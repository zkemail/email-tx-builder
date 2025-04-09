import { getContract, type Abi } from 'viem'
import { publicClients } from '../config/viemClient'
import { FACTORY_ADDRESSES } from '../config/chains'
import EmailSignerFactoryABI from '../abis/EmailSignerFactory.json'

const RELAYER_URL = process.env.RELAYER_URL || 'http://localhost:8000'

export async function calculateEthAddress(
    accountCode: string,
    email: string,
    chainId: number
): Promise<string> {
    if (!FACTORY_ADDRESSES[chainId]) {
        throw new Error(`Chain ID ${chainId} not supported`)
    }

    // Get the salt from relayer
    const { accountSalt } = await fetch(`${RELAYER_URL}/api/accountSalt`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            accountCode,
            emailAddress: email,
            chainId
        })
    }).then((res) => res.json())

    // Create contract instance
    const emailSignerFactory = getContract({
        address: FACTORY_ADDRESSES[chainId],
        abi: EmailSignerFactoryABI.abi,
        client: publicClients[chainId]
    })

    // Predict the address
    const emailSignerAddress = await emailSignerFactory.read.predictAddress([
        accountSalt
    ]) as `0x${string}`

    return emailSignerAddress
} 