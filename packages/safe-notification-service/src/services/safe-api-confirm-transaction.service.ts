import logger from '../utils/logger';
import { hexZeroPad, hexConcat, hexlify } from '@ethersproject/bytes';
import SafeApiKit from '@safe-global/api-kit';
import type { IConfirmTransactionService } from './confirm-transaction.interface';

export class SafeApiConfirmTransactionService implements IConfirmTransactionService {
    private safeApiKit: SafeApiKit;

    constructor(apiKit: SafeApiKit) {
        this.safeApiKit = apiKit;
    }

    private transformToSafeCompatibleSignature(signerAddress: string, rawSignature: string): string {
        // remove 0x prefix and divide by 2 for hex length
        const rawSignatureLength = rawSignature.slice(2).length / 2;

        // first 32 bytes - padded address
        // next 32 bytes - fixed data position
        // next 2 bytes - signature type
        // next 32 bytes - signature length
        // remaining bytes - signature data
        const signatureData = hexConcat([
            hexZeroPad(signerAddress, 32), // signer address
            hexZeroPad('0x41', 32), // signature offset
            '0x00', // signature type
            hexZeroPad(hexlify(rawSignatureLength), 32), // signature length
            rawSignature // signature data
        ]);

        return signatureData
    }

    async confirmTransaction(input: {
        hashToApprove: string,
        signatureData: string,
        safeAddress: string,
        ethAddress: string,
    }
    ): Promise<void> {
        logger.info('Confirming transaction', { hashToApprove: input.hashToApprove, safeAddress: input.safeAddress });

        const safeSignature = this.transformToSafeCompatibleSignature(
            input.ethAddress,
            input.signatureData
        );

        await this.safeApiKit.confirmTransaction(
            input.hashToApprove,
            safeSignature
        );
    }
} 