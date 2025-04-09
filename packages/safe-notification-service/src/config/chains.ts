import * as viemChains from 'viem/chains'
import type { Chain } from 'viem/chains'

interface ChainConfig {
    viemChain: keyof typeof viemChains
    factoryAddress: `0x${string}`
    rpcUrl: string
    privateKey: `0x${string}`
}

if (!process.env.FACTORY_ADDRESS) {
    throw new Error('FACTORY_ADDRESS environment variable is required')
}

if (!process.env.RPC_URL) {
    throw new Error('RPC_URL environment variable is required')
}

if (!process.env.PRIVATE_KEY) {
    throw new Error('PRIVATE_KEY environment variable is required')
}

// Type assertion for the imported JSON
const chainConfigurations = {
    11155111: {
      viemChain: "sepolia",
      factoryAddress: process.env.FACTORY_ADDRESS,
      rpcUrl: process.env.RPC_URL,
      privateKey: process.env.PRIVATE_KEY,
    },
  } as Record<string, ChainConfig>;

// Map of supported chain IDs to their configurations
export const SUPPORTED_CHAINS: Record<number, Chain> = Object.entries(chainConfigurations).reduce(
    (acc, [chainId, config]) => ({
        ...acc,
        [Number(chainId)]: viemChains[config.viemChain],
    }),
    {}
)

// Map of chain IDs to their factory addresses
export const FACTORY_ADDRESSES: Record<number, `0x${string}`> = Object.entries(chainConfigurations).reduce(
    (acc, [chainId, config]) => ({
        ...acc,
        [Number(chainId)]: config.factoryAddress,
    }),
    {}
)

// Map of chain IDs to their RPC URLs
export const RPC_URLS: Record<number, string> = Object.entries(chainConfigurations).reduce(
    (acc, [chainId, config]) => ({
        ...acc,
        [Number(chainId)]: config.rpcUrl,
    }),
    {}
)

export const PRIVATE_KEYS: Record<number, `0x${string}`> = Object.entries(chainConfigurations).reduce(
    (acc, [chainId, config]) => ({
        ...acc,
        [Number(chainId)]: config.privateKey,
    }),
    {}
)
