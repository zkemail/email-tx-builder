import SafeApiKit from '@safe-global/api-kit';
import prisma from '../config/database';
import logger from '../utils/logger';
import { getEmailSignature } from '../utils/emailSignature';
import type { IConfirmTransactionService } from './confirm-transaction.interface';



export class SafeNotificationService {
    private safeApiKit: SafeApiKit;
    private confirmTransactionService: IConfirmTransactionService;
    private pollingInterval?: NodeJS.Timer;

    constructor(safeApiKit: SafeApiKit, confirmTransactionService: IConfirmTransactionService) {
        this.safeApiKit = safeApiKit;
        this.confirmTransactionService = confirmTransactionService;
    }

    async startPolling(intervalSeconds = 60) {
        logger.info(`Starting safe polling with ${intervalSeconds} second interval`);
        this.pollingInterval = setInterval(() => this.pollSafes(), intervalSeconds * 1000);
    }

    async shutdown() {
        logger.info('Shutting down SafeNotificationService');
        if (this.pollingInterval) {
            clearInterval(this.pollingInterval);
            this.pollingInterval = undefined;
        }
        await prisma.$disconnect();
        logger.info('SafeNotificationService shutdown complete');
    }

    private async pollSafes() {
        try {
            logger.debug('Starting polling cycle');
            const accounts = await prisma.account.findMany();
            logger.info(`Found ${accounts.length} accounts to process`);

            for (const account of accounts) {
                logger.debug(`Processing account ${account.email} with ${account.safeAddresses.length} safes`);
                for (const safeAddress of account.safeAddresses) {
                    logger.debug(`Processing safe ${safeAddress} for account ${account.email}`);
                    await this.processSafe(safeAddress, { ...account, safeAddress });
                    logger.debug(`Completed processing safe ${safeAddress} for account ${account.email}`);
                }
                logger.debug(`Completed processing account ${account.email}`);
            }
            logger.debug('Completed polling cycle');
        } catch (error) {
            logger.error('Error in safe polling:', error);
        }
    }

    private async processSafe(safeAddress: string, account: { email: string; accountCode: string; chainId: number; ethAddress: string; safeAddress: string }) {
        try {
            logger.debug(`Fetching pending transactions for safe ${safeAddress} on chain ${account.chainId}`);
            const pendingTxs = await this.safeApiKit.getPendingTransactions(safeAddress);
            logger.info(`Found ${pendingTxs.results.length} pending transactions for safe ${safeAddress}`);

            for (const tx of pendingTxs.results) {
                logger.debug(`Processing transaction ${tx.safeTxHash} for safe ${safeAddress}`);
                const processedTx = await prisma.safeTransaction.findUnique({
                    where: {
                        safeTxHash_chainId: {
                            safeTxHash: tx.safeTxHash,
                            chainId: account.chainId
                        },
                        processed: true
                    }
                });
                logger.debug(`Found processed transaction: ${processedTx} for safe ${safeAddress}`);

                if (processedTx) {
                    logger.debug(`Skipping already processed transaction ${tx.safeTxHash} for safe ${safeAddress}`);
                    continue;
                }

                logger.debug(`Processing transaction ${tx.safeTxHash} for safe ${safeAddress}`);
                await this.processTransaction(tx, safeAddress, account);
                logger.debug(`Transaction ${tx.safeTxHash} processed successfully for safe ${safeAddress}`);
            }
        } catch (error) {
            logger.error(`Error processing safe ${safeAddress}:`, error);
        }
    }

    private async processTransaction(
        tx: { safeTxHash: string },
        safeAddress: string,
        account: { email: string; accountCode: string; chainId: number; ethAddress: string; safeAddress: string }
    ) {
        try {
            await prisma.safeTransaction.create({
                data: {
                    safeTxHash: tx.safeTxHash,
                    safeAddress,
                    chainId: account.chainId,
                    processed: false
                }
            });

            const signatureData = await getEmailSignature(
                account.email,
                account.accountCode,
                account.ethAddress,
                tx.safeTxHash,
                account.chainId
            );

            await prisma.safeTransaction.upsert({
                where: {
                    safeTxHash_chainId: {
                        safeTxHash: tx.safeTxHash,
                        chainId: account.chainId
                    }
                },
                create: {
                    safeTxHash: tx.safeTxHash,
                    safeAddress,
                    chainId: account.chainId,
                    processed: false,
                    signature: signatureData
                },
                update: {
                    signature: signatureData
                }
            });
            logger.info('Generated and saved signature for hash', { hashToApprove: tx.safeTxHash });

            await this.confirmTransactionService.confirmTransaction(
                {
                    hashToApprove: tx.safeTxHash,
                    signatureData,
                    safeAddress,
                    ethAddress: account.ethAddress
                }
            );

            logger.debug(`Updating transaction record for ${tx.safeTxHash}`);
            await prisma.safeTransaction.update({
                where: {
                    safeTxHash_chainId: {
                        safeTxHash: tx.safeTxHash,
                        chainId: account.chainId
                    }
                },
                data: {
                    processed: true
                }
            });
            logger.info(`Transaction ${tx.safeTxHash} processed successfully`);

        } catch (error) {
            logger.error(`Error processing transaction ${tx.safeTxHash}:`, {
                error: error instanceof Error ? {
                    message: error.message,
                    stack: error.stack
                } : String(error),
                safeTxHash: tx.safeTxHash,
                safeAddress,
                chainId: account.chainId
            });
        }
    }
} 