import { encodeAbiParameters, getContract } from 'viem';
import { publicClients } from '../config/viemClient';
import EMAIL_SIGNER_ABI from '../abis/EmailSigner.json';
import logger from './logger';
import { PrismaClient } from '@prisma/client';

interface EmailProofParams {
    emailAddress: string;
    accountCode: string;
    txHashToSign: bigint;
    templateId: string;
}

interface EmailProofResponse {
    id: string;
    // Add other expected response properties here
}

export async function getEmailSignature(
    emailAddress: string,
    accountCode: string,
    ethAddress: string,
    safeTxHash: string,
    chainId: number
): Promise<string> {
    const client = publicClients[chainId];
    if (!client) {
        throw new Error(`No client found for chain ID ${chainId}`);
    }

    const txHashToSign = BigInt(safeTxHash);
    const emailSigner = getContract({
        address: ethAddress as `0x${string}`,
        abi: EMAIL_SIGNER_ABI,
        client
    });

    const templateId = `0x${((await emailSigner.read.templateId()) as bigint).toString(16)}`;

    logger.debug('Requesting signature', {
        templateId,
        dkimRegistryAddr: await emailSigner.read.dkimRegistryAddr(),
        txHashToSign: txHashToSign.toString()
    });

    const emailProofResponse = await getCachedOrFreshEmailProof({
        emailAddress,
        accountCode,
        txHashToSign,
        templateId
    });

    // wait for the proof to be generated
    const emailAuthMsg = await pollForProof(emailProofResponse.id);

    const smartContractSignature = encodeAbiParameters(
        [
            {
                type: 'tuple',
                components: [
                    { type: 'uint256', name: 'templateId' },
                    { type: 'bytes[]', name: 'commandParams' },
                    { type: 'uint256', name: 'skippedCommandPrefix' },
                    {
                        type: 'tuple',
                        name: 'proof',
                        components: [
                            { type: 'string', name: 'domainName' },
                            { type: 'bytes32', name: 'publicKeyHash' },
                            { type: 'uint256', name: 'timestamp' },
                            { type: 'string', name: 'maskedCommand' },
                            { type: 'bytes32', name: 'emailNullifier' },
                            { type: 'bytes32', name: 'accountSalt' },
                            { type: 'bool', name: 'isCodeExist' },
                            { type: 'bytes', name: 'proof' }
                        ]
                    }
                ]
            }
        ],
        [
            {
                templateId: emailAuthMsg.templateId,
                commandParams: emailAuthMsg.commandParams,
                skippedCommandPrefix: emailAuthMsg.skippedCommandPrefix,
                proof: emailAuthMsg.proof
            }
        ]
    );

    logger.info('Email signature', { signature: smartContractSignature });
    return smartContractSignature;
}

/**
 * @notice This function sends an email to the user requesting their signature. To avoid spamming users,
 * ensure this is only called once per unique set of arguments (emailAddress, accountCode, txHashToSign, templateId).
 * Multiple calls with the same arguments will result in duplicate emails being sent to the user.
 * 
 * @dev The function interacts with an external email service to generate proofs. Each call initiates a new
 * email flow, regardless of whether a proof was previously generated for the same parameters.
 */
async function requestSignature(
    emailAddress: string,
    accountCode: string,
    txHashToSign: bigint,
    templateId: string
) {
    const apiResponse = await fetch(`${process.env.RELAYER_URL}/api/submit`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            accountCode,
            codeExistsInEmail: true,
            commandTemplate: 'signHash {uint}',
            commandParams: [txHashToSign.toString()],
            templateId,
            emailAddress,
            subject: 'Safe Transaction Signature Request',
            body: `Please sign the safe transaction`
        })
    });

    if (!apiResponse.ok) {
        throw new Error(`Failed to get email signature: ${await apiResponse.text()}`);
    }
    const relayerResponse = await apiResponse.json(); // might be a better way to do this
    return relayerResponse
}

async function pollForProof(emailProofId: string) {
    let emailAuthMsg;
    let retries = 0;
    const maxRetries = 100;

    while (!emailAuthMsg && retries < maxRetries) {
        try {
            const statusResponse = await fetch(`${process.env.RELAYER_URL}/api/status/${emailProofId}`);

            if (!statusResponse.ok) {
                throw new Error(`Failed to get proof status: ${await statusResponse.text()}`);
            }

            const status = await statusResponse.json();

            if (status.error) {
                throw new Error(`Error getting proof: ${status.error}`);
            }

            if (status.response) {
                return status.response;
            }

            retries++;
            await new Promise((resolve) => setTimeout(resolve, 2000));
        } catch (error) {
            retries++;
            await new Promise((resolve) => setTimeout(resolve, 2000));
        }
    }

    throw new Error('Timed out waiting for email proof');
}

async function getCachedOrFreshEmailProof({
    emailAddress,
    accountCode,
    txHashToSign,
    templateId
}: EmailProofParams): Promise<EmailProofResponse> {
    const prisma = new PrismaClient();

    try {
        const proofCacheKey = [
            emailAddress.toLowerCase(), // Normalize email
            accountCode,
            txHashToSign.toString(),
            templateId.toLowerCase() // Normalize hex
        ].join('-');

        const cachedProof = await prisma.responseCache.findUnique({
            where: { cacheKey: proofCacheKey },
            select: { response: true, expiresAt: true }
        }) as { response: EmailProofResponse | null, expiresAt: Date };

        if (cachedProof && cachedProof.response && cachedProof.expiresAt > new Date()) {
            return cachedProof.response as EmailProofResponse;
        }

        const emailProofResponse = await requestSignature(
            emailAddress,
            accountCode,
            txHashToSign,
            templateId
        );

        const ONE_HOUR_MS = 60 * 60 * 1000;
        await prisma.responseCache.upsert({
            where: { cacheKey: proofCacheKey },
            update: {
                response: emailProofResponse,
                expiresAt: new Date(Date.now() + ONE_HOUR_MS)
            },
            create: {
                cacheKey: proofCacheKey,
                response: emailProofResponse,
                expiresAt: new Date(Date.now() + ONE_HOUR_MS)
            }
        });

        return emailProofResponse;
    } finally {
        await prisma.$disconnect();
    }
} 