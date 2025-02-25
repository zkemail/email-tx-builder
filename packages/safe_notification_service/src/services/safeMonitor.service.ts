import prisma from '../config/database';
import logger from '../utils/logger';
import { getEmailSignature } from '../utils/emailSignature';
import { buildContractSignature, buildSignatureBytes, EthSafeSignature } from '@safe-global/protocol-kit';
import SafeTransaction from '@safe-global/protocol-kit/dist/src/utils/transactions/SafeTransaction';
import { createPublicClient, createWalletClient, http, parseAbiItem } from 'viem';
import { publicClients } from '@/config/viemClient';
import { RPC_URLS, SUPPORTED_CHAINS } from '@/config/chains';
import { privateKeyToAccount } from 'viem/accounts';
import { Queue, Worker } from 'bullmq';

export class SafeMonitorService {
    private intervalId: NodeJS.Timer | null = null;
    private lastProcessTime = Date.now();
    private readonly RATE_LIMIT = 5; // 5 requests per second
    private readonly INTERVAL = 1000; // 1 second in milliseconds

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

    async queueSafeProcessingTasks() {
        try {
            const accountsWithSafes = await prisma.account.findMany({
                select: {
                    email: true,
                    accountCode: true,
                    safeAddresses: true,
                    chainId: true,
                    ethAddress: true
                },
                distinct: ['email', 'accountCode']
            });

            logger.info(`Found ${accountsWithSafes.length} accounts with safes to process`);

            accountsWithSafes.forEach(async (account) => {
                account.safeAddresses.forEach(async (safeAddress) => {
                    try {
                        await this.processSafe(account.email, account.accountCode, account.ethAddress, safeAddress, account.chainId);
                    } catch (error) {
                        logger.error('Error processing safe', { error });
                    }
                });
            });

        } catch (error) {
            logger.error('Error queuing safe processing tasks:', { error });
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


        const publicClient = createPublicClient({
            chain: SUPPORTED_CHAINS[chainId],
            transport: http(RPC_URLS[chainId])
        });

        publicClient.watchEvent({
            address: safeAddress,
            event: parseAbiItem('event ApproveHash(bytes32 hashToApprove, address owner)'),
            onLogs: async (logs) => {
                logs.forEach(async (log) => {
                    const hashToApprove = log.topics[1]!;
                    const owner = `0x${log.topics[2]!.slice(-40)}`;
                    console.log(hashToApprove, owner);

                    // now make sure the email signer has not already signed this hash
                    const isApproved = await publicClient.readContract({
                        address: safeAddress,
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
                        args: [ethAddress, hashToApprove]
                    });

                    if (isApproved) {
                        console.log(`Already approved`);
                        return;
                    } else {
                        await this.approveHash(hashToApprove, email, accountCode, safeAddress, ethAddress, chainId);
                    }
                });
            }
        });

    }

    async approveHash(
        hashToApprove: string,
        email: string,
        accountCode: string,
        safeAddress: string,
        ethAddress: string, // eth address of email signer
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

            const walletClient = createWalletClient({
                chain: SUPPORTED_CHAINS[chainId],
                transport: http(RPC_URLS[chainId]),
                account: privateKeyToAccount(`0x${process.env.PRIVATE_KEY!}`)
            });

            const hash = await walletClient.writeContract({
                address: ethAddress,
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
            logger.info('Approval transaction hash:', { hash });

        } catch (error) {
            logger.error('Error processing hash', {
                hashToApprove,
                safeAddress,
                error: error instanceof Error ? error.message : 'Unknown error'
            });
        }
    }

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