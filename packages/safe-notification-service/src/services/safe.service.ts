import prisma from '../config/database';
import logger from '../utils/logger';
import { type PublicClient } from 'viem';
import { SafeWatcher } from './safe-watcher';
import { HashApprovalService } from './hash-approval.service';

// Fix for the Timer vs Timeout type issue
type SafeTimeout = {
    [Symbol.dispose]?: () => void;
} & NodeJS.Timeout;

export class SafeService {
    private intervalId: NodeJS.Timer | null = null;
    private safeWatcher: SafeWatcher;
    private hashApprovalService: HashApprovalService;

    constructor() {
        const redisUrl = process.env.REDIS_URL!;
        this.safeWatcher = new SafeWatcher(redisUrl);
        this.hashApprovalService = new HashApprovalService();

        // Set up the event handler for ApproveHash events
        this.safeWatcher.onApproveHashEvent = this.handleApproveHashEvent.bind(this);
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

    // Delegate to SafeWatcher
    async setupSafeWatcher(
        email: string,
        accountCode: string,
        ethAddress: string,
        safeAddress: string,
        chainId: number
    ) {
        return this.safeWatcher.setupWatcher({
            email,
            accountCode,
            ethAddress,
            safeAddress,
            chainId
        });
    }

    // Event handling method
    private async handleApproveHashEvent(
        log: any,
        publicClient: PublicClient,
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

        const isApproved = await this.hashApprovalService.checkIfHashIsApproved(publicClient, {
            safeAddress,
            ethAddress,
            hashToApprove
        });

        if (!isApproved) {
            await this.hashApprovalService.approveHash(
                hashToApprove,
                email,
                accountCode,
                safeAddress,
                ethAddress,
                chainId
            );
        }
    }

    // Cleanup method
    async shutdown() {
        this.stop();
        await this.safeWatcher.shutdown();
        logger.info('Safe service shutdown complete');
    }
}