import prisma from '../config/database';
import SafeApiKit from '@safe-global/api-kit'
import logger from '../utils/logger';

export class SafeMonitorService {
    private intervalId: NodeJS.Timer | null = null;
    private queue: (() => Promise<void>)[] = [];
    private processing = false;
    private lastProcessTime = Date.now();
    private readonly RATE_LIMIT = 5; // 5 requests per second
    private readonly INTERVAL = 1000; // 1 second in milliseconds

    private async processQueue() {
        if (this.processing || this.queue.length === 0) return;

        this.processing = true;
        const now = Date.now();
        const timeToWait = Math.max(0, this.INTERVAL - (now - this.lastProcessTime));

        await new Promise(resolve => setTimeout(resolve, timeToWait));

        try {
            const batch = this.queue.splice(0, this.RATE_LIMIT);
            logger.debug(`Processing batch of ${batch.length} tasks`);
            await Promise.all(batch.map(task => task()));
            this.lastProcessTime = Date.now();
        } catch (error) {
            logger.error('Error processing queue:', { error });
        } finally {
            this.processing = false;
            if (this.queue.length > 0) {
                this.processQueue();
            }
        }
    }

    private async processSafe(email: string, accountCode: string, safeAddress: string, chainId: number) {
        try {
            logger.info('Processing safe', {
                email,
                accountCode,
                safeAddress,
                chainId
            });

            const safeApiKit = new SafeApiKit({
                chainId: BigInt(chainId)
            });

            const pendingTxs = await safeApiKit.getPendingTransactions(safeAddress);

            for (const tx of pendingTxs.results) {
                if (!tx.isExecuted && tx.confirmations) {
                    const txHash = tx.safeTxHash;
                    const requiredConfirmations = tx.confirmationsRequired;
                    const currentConfirmations = tx.confirmations.length;

                    logger.info('Found pending transaction', {
                        txHash,
                        safeAddress,
                        currentConfirmations,
                        requiredConfirmations
                    });
                }
            }
        } catch (error) {
            logger.error('Error processing safe', {
                safeAddress,
                chainId,
                error
            });
        }
    }

    async fetchAndProcessSafes() {
        try {
            const accounts = await prisma.account.findMany({
                select: {
                    email: true,
                    accountCode: true,
                    safeAddresses: true,
                    chainId: true
                },
                distinct: ['email', 'accountCode']
            });

            logger.info(`Found ${accounts.length} accounts to process`);

            accounts.forEach(account => {
                account.safeAddresses.forEach(safeAddress => {
                    this.queue.push(async () => {
                        await this.processSafe(account.email, account.accountCode, safeAddress, account.chainId);
                    });
                });
            });

            logger.debug(`Added ${this.queue.length} tasks to the queue`);
            this.processQueue();
        } catch (error) {
            logger.error('Error fetching safes:', { error });
        }
    }

    start(intervalMs: number = 10000) {
        if (this.intervalId) {
            logger.warn('Monitor already running');
            return;
        }

        logger.info('Starting Safe monitor', { intervalMs });
        this.fetchAndProcessSafes();
        this.intervalId = setInterval(() => {
            this.fetchAndProcessSafes();
        }, intervalMs);
    }

    stop() {
        if (this.intervalId) {
            clearInterval(this.intervalId);
            this.intervalId = null;
            logger.info('Safe monitor stopped');
        }
    }
} 