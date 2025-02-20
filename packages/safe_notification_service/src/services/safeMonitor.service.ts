import prisma from '../config/database';
import SafeApiKit from '@safe-global/api-kit'
import logger from '../utils/logger';
import { getEmailSignature } from '../utils/emailSignature';

export class SafeMonitorService {
    private intervalId: NodeJS.Timer | null = null;
    private queue: (() => Promise<void>)[] = [];
    private processing = false;
    private lastProcessTime = Date.now();
    private readonly RATE_LIMIT = 5; // 5 requests per second
    private readonly INTERVAL = 1000; // 1 second in milliseconds

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

    async fetchAndProcessSafes() {
        try {
            const accounts = await prisma.account.findMany({
                select: {
                    email: true,
                    accountCode: true,
                    safeAddresses: true,
                    chainId: true,
                    ethAddress: true
                },
                distinct: ['email', 'accountCode']
            });

            logger.info(`Found ${accounts.length} accounts to process`);

            accounts.forEach(account => {
                account.safeAddresses.forEach(safeAddress => {
                    this.queue.push(async () => {
                        await this.processSafe(account.email, account.accountCode, account.ethAddress, safeAddress, account.chainId);
                    });
                });
            });

            logger.debug(`Added ${this.queue.length} tasks to the queue`);
            this.processQueue();
        } catch (error) {
            logger.error('Error fetching safes:', { error });
        }
    }

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
            throw error;
        } finally {
            this.processing = false;
            if (this.queue.length > 0) {
                this.processQueue();
            }
        }
    }

    private async processSafe(email: string, accountCode: string, ethAddress: string, safeAddress: string, chainId: number) {
        logger.info('Processing safe', {
            email,
            accountCode,
            ethAddress,
            safeAddress,
            chainId
        });

        try {
            const safeApiKit = new SafeApiKit({
                chainId: BigInt(chainId)
            });

            const pendingTxs = await safeApiKit.getPendingTransactions(safeAddress);
            logger.debug('Found pending transactions', {
                count: pendingTxs.results.length,
                safeAddress
            });

            for (const tx of pendingTxs.results) {
                if (!tx.isExecuted && tx.confirmations) {
                    await this.processTransaction(tx, email, accountCode, safeAddress, ethAddress, chainId);
                }
            }
        } catch (error) {
            logger.error('Error processing safe', {
                safeAddress,
                chainId,
                error
            });
            throw error;
        }
    }

    private async processTransaction(
        tx: any,
        email: string,
        accountCode: string,
        safeAddress: string,
        ethAddress: string,
        chainId: number
    ) {
        const txHash = tx.safeTxHash;
        logger.debug('Starting transaction processing', { txHash, safeAddress });

        try {
            const existingTx = await prisma.safeTransaction.findUnique({
                where: {
                    safeTxHash_chainId: {
                        safeTxHash: txHash,
                        chainId
                    }
                }
            });

            // If we already have a signature, we've successfully called the API before
            if (existingTx?.signature) {
                logger.debug('Transaction already has signature', { txHash });
                return;
            }

            // Create or update transaction record
            if (!existingTx) {
                await prisma.safeTransaction.create({
                    data: {
                        safeTxHash: txHash,
                        safeAddress,
                        chainId,
                        processed: false
                    }
                });
            }


            logger.info('Processing pending transaction', {
                txHash,
                safeAddress,
                currentConfirmations: tx.confirmations.length,
                requiredConfirmations: tx.confirmationsRequired,
                isNew: !existingTx
            });

            // Generate signature using the utility function
            const signature = await getEmailSignature(
                email,
                accountCode,
                ethAddress,
                txHash,
                chainId
            );

            // Update transaction with signature
            await prisma.safeTransaction.update({
                where: {
                    safeTxHash_chainId: {
                        safeTxHash: txHash,
                        chainId
                    }
                },
                data: {
                    signature: signature.data,
                    processed: true
                }
            });

            logger.info('Generated and saved signature for transaction', { txHash });
        } catch (error) {
            logger.error('Error processing transaction', {
                txHash,
                safeAddress,
                error
            });
            throw error;
        }
    }
} 