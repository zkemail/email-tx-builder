import { OperationType, type SafeMultisigTransactionResponse } from "@safe-global/types-kit";

export class TransactionCheckerService {
    constructor() { }

    private isDelegateCall = (tx: SafeMultisigTransactionResponse): boolean => {
        return tx.operation === OperationType.DelegateCall;
    };

    async getWarnings(tx: SafeMultisigTransactionResponse): Promise<string[]> {
        const warnings = [];

        if (this.isDelegateCall(tx)) {
            warnings.push('This is a delegate call. Please be aware that this transaction may be malicious.');
        }

        return warnings;
    }
}
