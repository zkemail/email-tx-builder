import { createPublicClient, http, type PublicClient } from 'viem'
import { SUPPORTED_CHAINS, RPC_URLS } from './chains'

// Create a client for each supported chain
export const publicClients: Record<number, PublicClient> = Object.entries(SUPPORTED_CHAINS).reduce(
    (clients, [chainId, chain]) => ({
        ...clients,
        [chainId]: createPublicClient({
            chain,
            transport: http(RPC_URLS[Number(chainId)])
        })
    }),
    {}
) 