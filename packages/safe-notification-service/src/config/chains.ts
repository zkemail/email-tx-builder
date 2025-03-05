import * as viemChains from 'viem/chains'
import chainsConfig from './chains.json'
import type { Chain } from 'viem/chains'

interface ChainConfig {
    viemChain: keyof typeof viemChains
    factoryAddress: `0x${string}`
    rpcUrl: string
    privateKey: `0x${string}`
}

// Type assertion for the imported JSON
const chainConfigurations = chainsConfig as Record<string, ChainConfig>

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
