import prisma from '../config/database';
import logger from '../utils/logger';
import { getEmailSignature } from '../utils/emailSignature';
import { createPublicClient, createWalletClient, http, parseAbiItem } from 'viem';
import { RPC_URLS, SUPPORTED_CHAINS, PRIVATE_KEYS } from '@/config/chains';
import { privateKeyToAccount } from 'viem/accounts';
import { DistributedLock } from '../utils/distributedLock';
import { v4 as uuidv4 } from 'uuid';

export class SafeService {
    private intervalId: NodeJS.Timer | null = null;
    private lock: DistributedLock;
    private activeWatchdogs: Map<string, NodeJS.Timeout> = new Map();

    constructor() {
        this.lock = new DistributedLock(process.env.REDIS_URL!);
    }

    // Service lifecycle methods
    start(intervalMs: number = 10000) {
        if (this.intervalId) {
            logger.warn('Monitor already running, stopping previous instance first');
            this.stop();
        }

        logger.info('Starting Safe monitor', { intervalMs });
        this.queueSafeProcessingTasks();
        this.intervalId = setInterval(() => {
            this.queueSafeProcessingTasks();
        }, intervalMs);
    }

    stop() {
        if (this.intervalId) {
            clearInterval(this.intervalId);
            this.intervalId = null;
            logger.info('Safe monitor stopped');
        }
    }

    // Main processing methods
    async queueSafeProcessingTasks() {
        try {
            const accountsWithSafes = await prisma.account.findMany({
                select: {
                    email: true,
                    accountCode: true,
                    ethAddress: true,
                    safeAddresses: true,
                    chainId: true
                },
                distinct: ['email', 'accountCode']
            });

            logger.info(`Found ${accountsWithSafes.length} accounts with safes to process`);

            await Promise.all(
                accountsWithSafes.flatMap(account =>
                    account.safeAddresses.map(safeAddress =>
                        this.setupSafeWatcher(
                            account.email,
                            account.accountCode,
                            account.ethAddress,
                            safeAddress,
                            account.chainId
                        ).catch(error => {
                            logger.error('Error setting up safe watcher', {
                                error: error instanceof Error ? {
                                    message: error.message,
                                    stack: error.stack
                                } : 'Unknown error',
                                safeAddress
                            });
                        })
                    )
                )
            );
        } catch (error) {
            logger.error('Error queuing safe processing tasks:', {
                error: error instanceof Error ? {
                    message: error.message,
                    stack: error.stack
                } : 'Unknown error'
            });
        }
    }

    // Simpler approach that directly sets up watchers if needed
    async setupSafeWatcher(email: string, accountCode: string, ethAddress: string, safeAddress: string, chainId: number) {
        const watcherKey = `watcher:${chainId}:${safeAddress}`;

        // Check if already watching
        if (this.activeWatchdogs.has(watcherKey)) {
            logger.debug('Already watching this safe locally', { safeAddress, chainId });
            return;
        }

        // Try to acquire a Redis lock for this watcher - never expires
        const watcherId = uuidv4();
        const acquired = await this.lock.client.set(watcherKey, watcherId, 'NX');

        if (acquired !== 'OK') {
            // Someone else is already watching this safe
            logger.debug('Safe already being watched by another instance', { safeAddress, chainId });
            return;
        }

        try {
            // We got the watcher lock, set up the event watcher
            logger.info('Setting up new watcher for safe', { safeAddress, chainId });
            const publicClient = this.createPublicClient(chainId);
            this.setupEventWatcher(publicClient, {
                email,
                accountCode,
                ethAddress,
                safeAddress,
                chainId,
                watcherId,
                watcherKey
            });
        } catch (error) {
            // Release the watcher lock on error
            await this.lock.client.del(watcherKey);
            logger.error('Failed to set up watcher', {
                error: error instanceof Error ? {
                    message: error.message,
                    stack: error.stack
                } : 'Unknown error',
                safeAddress,
                chainId
            });
        }
    }

    // Client creation helpers
    private createPublicClient(chainId: number) {
        return createPublicClient({
            chain: SUPPORTED_CHAINS[chainId],
            transport: http(RPC_URLS[chainId])
        });
    }

    private createWalletClient(chainId: number) {
        return createWalletClient({
            chain: SUPPORTED_CHAINS[chainId],
            transport: http(RPC_URLS[chainId]),
            account: privateKeyToAccount(`0x${PRIVATE_KEYS[chainId]}`)
        });
    }

    // Simplified event watcher setup
    private setupEventWatcher(
        publicClient: any,
        params: {
            email: string;
            accountCode: string;
            ethAddress: string;
            safeAddress: string;
            chainId: number;
            watcherId: string;
            watcherKey: string;
        }
    ) {
        const { email, accountCode, ethAddress, safeAddress, chainId, watcherId, watcherKey } = params;

        // Set up heartbeat to ensure we still own the watcher
        const watchdogInterval = setInterval(async () => {
            try {
                // Check if we still own the watcher
                const currentWatcherId = await this.lock.client.get(watcherKey);
                if (currentWatcherId !== watcherId) {
                    logger.warn('Lost ownership of watcher, shutting down', {
                        safeAddress,
                        chainId,
                        expected: watcherId,
                        actual: currentWatcherId
                    });

                    // Clean up our watcher
                    clearInterval(watchdogInterval);
                    this.activeWatchdogs.delete(watcherKey);
                    return;
                }
            } catch (error) {
                logger.error('Error in watcher heartbeat', {
                    error: error instanceof Error ? error.message : 'Unknown error',
                    safeAddress,
                    chainId
                });
            }
        }, 30000);

        // Store the interval for cleanup
        this.activeWatchdogs.set(watcherKey, watchdogInterval);

        // Set up the event watcher
        const unwatch = publicClient.watchEvent({
            address: safeAddress as `0x${string}`,
            event: parseAbiItem('event ApproveHash(bytes32 hashToApprove, address owner)'),
            onLogs: async (logs: any) => {
                for (const log of logs) {
                    await this.handleApproveHashEvent(log, publicClient, {
                        email,
                        accountCode,
                        ethAddress,
                        safeAddress,
                        chainId
                    });
                }
            },
            onError: (error: any) => {
                logger.error('Error in event watcher', {
                    error: error instanceof Error ? error.message : 'Unknown error',
                    safeAddress,
                    chainId
                });

                // Clean up on error
                this.cleanupWatcher(watcherKey, watcherId, unwatch, watchdogInterval);
            }
        });

        logger.info('Event watcher setup complete');

        // Set up cleanup handler for shutdown
        process.on('beforeExit', () => {
            this.cleanupWatcher(watcherKey, watcherId, unwatch, watchdogInterval);
        });
    }

    // Helper to cleanup watchers
    private async cleanupWatcher(
        watcherKey: string,
        watcherId: string,
        unwatch: () => void,
        interval: NodeJS.Timeout
    ) {
        try {
            // Only delete if we still own it
            const currentWatcherId = await this.lock.client.get(watcherKey);
            if (currentWatcherId === watcherId) {
                await this.lock.client.del(watcherKey);
            }

            unwatch();
            clearInterval(interval);
            this.activeWatchdogs.delete(watcherKey);
            logger.debug('Watcher cleaned up', { watcherKey });
        } catch (error) {
            logger.error('Error cleaning up watcher', {
                error: error instanceof Error ? error.message : 'Unknown error',
                watcherKey
            });
        }
    }

    // Event handling methods
    private async handleApproveHashEvent(
        log: any,
        publicClient: any,
        params: {
            email: string;
            accountCode: string;
            ethAddress: string;
            safeAddress: string;
            chainId: number;
        }
    ) {
        const { email, accountCode, ethAddress, safeAddress, chainId } = params;
        const hashToApprove = log.topics[1]!;
        const owner = `0x${log.topics[2]!.slice(-40)}`;
        logger.debug('ApproveHash event detected', { hashToApprove, owner });

        const isApproved = await this.checkIfHashIsApproved(publicClient, {
            safeAddress,
            ethAddress,
            hashToApprove
        });

        if (!isApproved) {
            await this.approveHash(hashToApprove, email, accountCode, safeAddress, ethAddress, chainId);
        }
    }

    // Hash approval methods
    private async checkIfHashIsApproved(
        publicClient: any,
        params: {
            safeAddress: string;
            ethAddress: string;
            hashToApprove: string;
        }
    ) {
        const { safeAddress, ethAddress, hashToApprove } = params;
        const isApproved = await publicClient.readContract({
            address: safeAddress as `0x${string}`,
            abi: [{
                name: 'approvedHashes',
                type: 'function',
                inputs: [
                    { type: 'address', name: 'owner' },
                    { type: 'bytes32', name: 'hash' }
                ],
                outputs: [{ type: 'uint256', name: 'approved' }],
                stateMutability: 'view'
            }],
            functionName: 'approvedHashes',
            args: [ethAddress as `0x${string}`, hashToApprove]
        });

        if (isApproved) {
            logger.debug('Hash already approved', { hashToApprove });
        }
        return isApproved;
    }

    async approveHash(
        hashToApprove: string,
        email: string,
        accountCode: string,
        safeAddress: string,
        ethAddress: string,
        chainId: number
    ) {
        logger.info('Approving hash', { hashToApprove, safeAddress });

        try {
            const signatureData = await this.getOrCreateSignature(
                hashToApprove,
                email,
                accountCode,
                ethAddress,
                safeAddress,
                chainId
            );

            const walletClient = this.createWalletClient(chainId);
            const hash = await this.submitApproveHashTransaction(
                walletClient,
                ethAddress,
                hashToApprove,
                signatureData,
                safeAddress
            );

            logger.info('Approval transaction hash:', { hash });
        } catch (error) {
            logger.error('Error processing hash', {
                hashToApprove,
                safeAddress,
                error: error instanceof Error ? {
                    message: error.message,
                    stack: error.stack
                } : 'Unknown error'
            });
        }
    }

    private async submitApproveHashTransaction(
        walletClient: any,
        ethAddress: string,
        hashToApprove: string,
        signatureData: string,
        safeAddress: string
    ) {
        return walletClient.writeContract({
            address: ethAddress as `0x${string}`,
            abi: [{
                name: 'approveHash',
                type: 'function',
                inputs: [
                    { type: 'bytes32', name: 'hashToApprove' },
                    { type: 'bytes', name: 'signature' },
                    { type: 'address', name: 'safe' }
                ],
                outputs: [],
                stateMutability: 'external'
            }],
            functionName: 'approveHash',
            args: [hashToApprove, signatureData, safeAddress]
        });
    }

    // Signature management
    async getOrCreateSignature(
        hashToApprove: string,
        email: string,
        accountCode: string,
        ethAddress: string,
        safeAddress: string,
        chainId: number
    ): Promise<string> {
        const existingTx = await prisma.safeTransaction.findUnique({
            where: {
                safeTxHash_chainId: {
                    safeTxHash: hashToApprove,
                    chainId
                }
            }
        });

        if (existingTx?.signature) {
            logger.info('Using existing signature', { hashToApprove });
            return existingTx.signature;
        }

        logger.info('Generating new signature', {
            hashToApprove,
            safeAddress,
        });

        const emailSignature = await getEmailSignature(
            email,
            accountCode,
            ethAddress,
            hashToApprove,
            chainId
        );

        await prisma.safeTransaction.upsert({
            where: {
                safeTxHash_chainId: {
                    safeTxHash: hashToApprove,
                    chainId
                }
            },
            create: {
                safeTxHash: hashToApprove,
                safeAddress,
                chainId,
                processed: false,
                signature: emailSignature
            },
            update: {
                signature: emailSignature
            }
        });

        logger.info('Generated and saved signature for hash', { hashToApprove });
        return emailSignature;
    }

    // Update shutdown to use new cleanup method
    async shutdown() {
        this.stop();

        // Clean up all active watchers
        const cleanupPromises = Array.from(this.activeWatchdogs.entries()).map(async ([key, interval]) => {
            try {
                clearInterval(interval);

                // Only delete if we still own it
                const watcherId = await this.lock.client.get(key);
                if (watcherId) {
                    await this.lock.client.del(key);
                }

                this.activeWatchdogs.delete(key);
            } catch (e) {
                logger.error(`Failed to clean up watcher ${key}`, {
                    error: e instanceof Error ? e.message : 'Unknown error'
                });
            }
        });

        await Promise.all(cleanupPromises);
        await this.lock.cleanup();
        logger.info('Safe service shutdown complete');
    }
}