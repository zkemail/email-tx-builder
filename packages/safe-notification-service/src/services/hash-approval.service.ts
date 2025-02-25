import { createWalletClient, http, type PublicClient } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { PRIVATE_KEYS, RPC_URLS, SUPPORTED_CHAINS } from '@/config/chains';
import logger from '../utils/logger';
import { getEmailSignature } from '../utils/emailSignature';
import prisma from '../config/database';

export class HashApprovalService {
    async checkIfHashIsApproved(
        publicClient: PublicClient,
        params: {
            safeAddress: string;
            ethAddress: string;
            hashToApprove: string;
        }
    ): Promise<boolean> {
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
            args: [ethAddress as `0x${string}`, hashToApprove as `0x${string}`]
        });

        if (isApproved) {
            logger.debug('Hash already approved', { hashToApprove });
        }
        return !!isApproved;
    }

    async approveHash(
        hashToApprove: string,
        email: string,
        accountCode: string,
        safeAddress: string,
        ethAddress: string,
        chainId: number
    ): Promise<string | undefined> {
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
            return hash;
        } catch (error) {
            logger.error('Error processing hash', {
                hashToApprove,
                safeAddress,
                error: error instanceof Error ? {
                    message: error.message,
                    stack: error.stack
                } : 'Unknown error'
            });
            return undefined;
        }
    }

    private createWalletClient(chainId: number) {
        return createWalletClient({
            chain: SUPPORTED_CHAINS[chainId],
            transport: http(RPC_URLS[chainId]),
            account: privateKeyToAccount(`0x${PRIVATE_KEYS[chainId]}`)
        });
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