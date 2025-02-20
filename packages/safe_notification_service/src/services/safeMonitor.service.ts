import prisma from '../config/database';
import SafeApiKit from '@safe-global/api-kit'


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
            await Promise.all(batch.map(task => task()));
            this.lastProcessTime = Date.now();
        } catch (error) {
            console.error('Error processing queue:', error);
        } finally {
            this.processing = false;
            if (this.queue.length > 0) {
                this.processQueue();
            }
        }
    }

    private async processSafe(email: string, accountCode: string, safeAddress: string, chainId: number) {
        try {
            console.log(`Processing safe for:`);
            console.log(`Email: ${email}`);
            console.log(`Account Code: ${accountCode}`);
            console.log(`Safe Address: ${safeAddress}`);
            console.log(`Chain ID: ${chainId}`);
            const safeApiKit = new SafeApiKit({
                chainId: BigInt(chainId) // e.g. Sepolia = 11155111
            })

            // Get pending transactions for a specific safe address
            const pendingTxs = await safeApiKit.getPendingTransactions(safeAddress)

            // Loop through pending transactions and check confirmation status
            for (const tx of pendingTxs.results) {
                if (!tx.isExecuted && tx.confirmations) {
                    const txHash = tx.safeTxHash;
                    const requiredConfirmations = tx.confirmationsRequired;
                    const currentConfirmations = tx.confirmations.length;

                    console.log(`Found pending transaction ${txHash} for safe ${safeAddress}`);
                    console.log(`Confirmations: ${currentConfirmations}/${requiredConfirmations}`);
                }
            }

        } catch (error) {
            console.error(`Error processing safe ${safeAddress} on chain ${chainId}:`, error);
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

            // Add each safe processing task to the queue
            accounts.forEach(account => {
                account.safeAddresses.forEach(safeAddress => {
                    this.queue.push(async () => {
                        await this.processSafe(account.email, account.accountCode, safeAddress, account.chainId);
                    });
                });
            });

            // Start processing the queue
            this.processQueue();
        } catch (error) {
            console.error('Error fetching safes:', error);
        }
    }

    start(intervalMs: number = 10000) { // Default to 10 seconds
        if (this.intervalId) {
            console.warn('Monitor already running');
            return;
        }

        console.log('Starting Safe monitor...');
        this.fetchAndProcessSafes(); // Initial run
        this.intervalId = setInterval(() => {
            this.fetchAndProcessSafes();
        }, intervalMs);
    }

    stop() {
        if (this.intervalId) {
            clearInterval(this.intervalId);
            this.intervalId = null;
            console.log('Safe monitor stopped');
        }
    }
} 