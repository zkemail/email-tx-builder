import { encodeAbiParameters, getContract } from 'viem';
import { publicClients } from '../config/viemClient';
import EMAIL_SIGNER_ABI from '../abis/EmailSigner.json';
import logger from './logger';
import { PrismaClient } from '@prisma/client';

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

    const emailAuthMsg = await requestSignature(
        emailAddress,
        accountCode,
        txHashToSign,
        templateId
    );

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

async function requestSignature(
    emailAddress: string,
    accountCode: string,
    txHashToSign: bigint,
    templateId: string
) {
    let relayerResponse; // response from relayer - check cache first

    const db = new PrismaClient();
    const cacheKey = `${emailAddress}-${accountCode}-${txHashToSign}-${templateId}`;
    const cachedResponse = await db.responseCache.findUnique({
        where: {
            cacheKey
        }
    });

    if (cachedResponse) {
        relayerResponse = cachedResponse;
    } else {
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
        relayerResponse = await apiResponse.json();
        await db.responseCache.create({
            data: {
                cacheKey,
                response: relayerResponse,
                expiresAt: new Date(Date.now() + 60 * 60 * 1000) // 1 hour from now
            }
        });
    }

    return await pollForProof(relayerResponse.id);
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