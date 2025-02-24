import prisma from '../config/database';
import SafeApiKit from '@safe-global/api-kit'
import logger from '../utils/logger';
import { getEmailSignature } from '../utils/emailSignature';
import { buildContractSignature, buildSignatureBytes, EthSafeSignature } from '@safe-global/protocol-kit';
import SafeTransaction from '@safe-global/protocol-kit/dist/src/utils/transactions/SafeTransaction';

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

            logger.info(`Added ${this.queue.length} tasks to the queue`);
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
            logger.info(`Processing batch of ${batch.length} tasks`);
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

            const pendingTxsResponse = await safeApiKit.getPendingTransactions(safeAddress);

            // Filter out transactions that already have enough confirmations
            const pendingTxs = pendingTxsResponse.results.filter(tx =>
                !tx.isExecuted &&
                tx.confirmations &&
                tx.confirmations.length < tx.confirmationsRequired
            );

            logger.info('Found pending transactions', {
                count: pendingTxs.length,
                safeAddress
            });

            for (const tx of pendingTxs) {
                if (!tx.isExecuted && tx.confirmations && tx.confirmations.length < tx.confirmationsRequired) {
                    await this.processTransaction(tx, email, accountCode, safeAddress, ethAddress, chainId);
                }
            }
        } catch (error) {
            logger.error('Error processing safe', {
                safeAddress,
                chainId,
                error: error instanceof Error ? error.message : 'Unknown error'
            });
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
        logger.info('Starting transaction processing', { txHash, safeAddress });

        let signatureData;

        try {
            const existingTx = await prisma.safeTransaction.findUnique({
                where: {
                    safeTxHash_chainId: {
                        safeTxHash: txHash,
                        chainId
                    }
                }
            });

            if (existingTx?.signature) {
                logger.info('Using existing signature', { txHash });
                signatureData = existingTx.signature;
            } else {
                logger.info('Generating new signature', {
                    txHash,
                    safeAddress,
                    currentConfirmations: tx.confirmations?.length,
                    requiredConfirmations: tx.confirmationsRequired
                });

                const emailSignature = await getEmailSignature(
                    email,
                    accountCode,
                    ethAddress,
                    txHash,
                    chainId
                );

                await prisma.safeTransaction.upsert({
                    where: {
                        safeTxHash_chainId: {
                            safeTxHash: txHash,
                            chainId
                        }
                    },
                    create: {
                        safeTxHash: txHash,
                        safeAddress,
                        chainId,
                        processed: false,
                        signature: emailSignature
                    },
                    update: {
                        signature: emailSignature
                    }
                });

                signatureData = emailSignature;
                logger.info('Generated and saved signature for transaction', { txHash });
            }

            const safeApiKit = new SafeApiKit({
                chainId: BigInt(chainId)
            });

            // Create contract signature bytes
            const safeSignature = new EthSafeSignature(ethAddress, signatureData, true);
            const signatureBytes = buildSignatureBytes([safeSignature]);

            logger.info('Submitting contract signature', {
                txHash,
                signer: ethAddress,
                signatureBytes
            });
            await safeApiKit.confirmTransaction(txHash, signatureBytes);

            logger.info('Transaction confirmed', { txHash });

        } catch (error) {
            logger.error('Error processing transaction', {
                txHash,
                safeAddress,
                error: error instanceof Error ? error.message : 'Unknown error'
            });
        }
    }
} 