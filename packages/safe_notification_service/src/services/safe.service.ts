import prisma from '../config/database';
import logger from '../utils/logger';
import { getEmailSignature } from '../utils/emailSignature';
import { createPublicClient, createWalletClient, http, parseAbiItem } from 'viem';
import { RPC_URLS, SUPPORTED_CHAINS, PRIVATE_KEYS } from '@/config/chains';
import { privateKeyToAccount } from 'viem/accounts';
import { DistributedLock } from '../utils/distributedLock';

export class SafeService {
    private intervalId: NodeJS.Timer | null = null;
    private lock: DistributedLock;

    constructor() {
        this.lock = new DistributedLock(process.env.REDIS_URL!);
    }

    // Service lifecycle methods
    start(intervalMs: number = 10000) {
        if (this.intervalId) {
            logger.warn('Monitor already running');
            return;
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
                        this.processSafeWithLock(
                            account.email,
                            account.accountCode,
                            account.ethAddress,
                            safeAddress,
                            account.chainId
                        ).catch(error => {
                            logger.error('Error processing safe', {
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

    async processSafeWithLock(email: string, accountCode: string, ethAddress: string, safeAddress: string, chainId: number) {
        const lockKey = `safe:${chainId}:${safeAddress}`;

        try {
            // todo: properly handle the lock and don't add duplicates
            await this.lock.acquireLock(lockKey);
            await this.processSafe(email, accountCode, ethAddress, safeAddress, chainId);
        } catch (error) {
            logger.error('Error processing safe with lock', {
                error: error instanceof Error ? {
                    message: error.message,
                    stack: error.stack
                } : 'Unknown error',
                safeAddress
            });
        } finally {
            await this.lock.releaseLock(lockKey);
            logger.debug('Lock released for safe', { safeAddress, chainId });
        }
    }

    async processSafe(email: string, accountCode: string, ethAddress: string, safeAddress: string, chainId: number) {
        logger.info('Processing safe', {
            email,
            accountCode,
            ethAddress,
            safeAddress,
            chainId
        });

        const publicClient = this.createPublicClient(chainId);
        await this.setupEventWatcher(publicClient, {
            email,
            accountCode,
            ethAddress,
            safeAddress,
            chainId
        });
    }

    // Event handling methods
    private async setupEventWatcher(
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

        publicClient.watchEvent({
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
            }
        });
    }

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
}