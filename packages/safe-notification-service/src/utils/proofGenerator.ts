interface EmailProof {
    domainName: string;
    publicKeyHash: `0x${string}`;
    timestamp: bigint;
    maskedCommand: string;
    emailNullifier: `0x${string}`;
    accountSalt: `0x${string}`;
    isCodeExist: boolean;
    proof: `0x${string}`;
}

interface EmailAuthMsg {
    templateId: bigint;
    commandParams: `0x${string}`[];
    skippedCommandPrefix: bigint;
    proof: EmailProof;
}
