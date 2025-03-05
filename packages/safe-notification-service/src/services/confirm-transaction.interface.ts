/**
 * Interface for confirming Safe transactions
 */
export interface IConfirmTransactionService {
    confirmTransaction(input: {
        hashToApprove: string,
        signatureData: string,
        safeAddress: string,
        ethAddress: string,
    }): Promise<void>
} 