import logger from '../utils/logger';
import type { IConfirmTransactionService } from './confirm-transaction.interface';
import { PRIVATE_KEYS } from '../config/chains';
import { createWalletClient, http } from 'viem';
import type { WalletClient } from 'viem';
import { RPC_URLS } from '../config/chains';
import { SUPPORTED_CHAINS } from '../config/chains';
import { privateKeyToAccount } from 'viem/accounts';

export class OnChainConfirmTransactionService implements IConfirmTransactionService {
    private walletClient: WalletClient;

    constructor(chainId: number) {
        this.walletClient = createWalletClient({
            chain: SUPPORTED_CHAINS[chainId],
            transport: http(RPC_URLS[chainId]),
            account: privateKeyToAccount(PRIVATE_KEYS[chainId])
        });
    }
    async confirmTransaction(input: {
        hashToApprove: string,
        signatureData: string,
        safeAddress: string,
        ethAddress: string,
    }
    ): Promise<void> {
        logger.info('Confirming transaction', { hashToApprove: input.hashToApprove, safeAddress: input.safeAddress });

        await this.walletClient.writeContract({
            address: input.ethAddress as `0x${string}`,
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
            args: [input.hashToApprove, input.signatureData, input.safeAddress],
            chain: this.walletClient.chain,
            account: this.walletClient.account!
        });
    }
} 