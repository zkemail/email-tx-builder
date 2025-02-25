import { createPublicClient, http, parseAbiItem, type PublicClient } from 'viem';
import { RPC_URLS, SUPPORTED_CHAINS } from '@/config/chains';
import logger from '../utils/logger';
import { DistributedLock } from '../utils/distributedLock';
import { v4 as uuidv4 } from 'uuid';

interface WatcherParams {
    email: string;
    accountCode: string;
    ethAddress: string;
    safeAddress: string;
    chainId: number;
}

// Fix for the Timer vs Timeout type issue
type SafeTimeout = {
    [Symbol.dispose]?: () => void;
} & NodeJS.Timeout;

export class SafeWatcher {
    private activeWatchdogs: Map<string, SafeTimeout> = new Map();
    private lock: DistributedLock;

    constructor(redisUrl: string) {
        this.lock = new DistributedLock(redisUrl);
    }

    async setupWatcher(params: WatcherParams): Promise<boolean> {
        const { email, accountCode, ethAddress, safeAddress, chainId } = params;
        const watcherKey = this.getWatcherKey(chainId, safeAddress);

        // Check if already watching locally
        if (this.activeWatchdogs.has(watcherKey)) {
            logger.debug('Already watching this safe locally', { safeAddress, chainId });
            return false;
        }

        // Try to acquire a Redis lock for this watcher - never expires
        const watcherId = uuidv4();
        const acquired = await this.lock.client.set(watcherKey, watcherId, 'NX');

        if (acquired !== 'OK') {
            // Someone else is already watching this safe
            logger.debug('Safe already being watched by another instance', { safeAddress, chainId });
            return false;
        }

        try {
            // We got the watcher lock, set up the event watcher
            logger.info('Setting up new watcher for safe', { safeAddress, chainId });
            const publicClient = this.createPublicClient(chainId);

            this.setupEventWatcher(publicClient, {
                email,
                accountCode,
                ethAddress,
                safeAddress,
                chainId,
                watcherId,
                watcherKey
            });

            return true;
        } catch (error) {
            // Release the watcher lock on error
            await this.lock.client.del(watcherKey);
            logger.error('Failed to set up watcher', {
                error: error instanceof Error ? {
                    message: error.message,
                    stack: error.stack
                } : 'Unknown error',
                safeAddress,
                chainId
            });
            return false;
        }
    }

    private getWatcherKey(chainId: number, safeAddress: string): string {
        return `watcher:${chainId}:${safeAddress}`;
    }

    private createPublicClient(chainId: number): PublicClient {
        return createPublicClient({
            chain: SUPPORTED_CHAINS[chainId],
            transport: http(RPC_URLS[chainId])
        });
    }

    private setupEventWatcher(
        publicClient: PublicClient,
        params: WatcherParams & { watcherId: string; watcherKey: string }
    ) {
        const { email, accountCode, ethAddress, safeAddress, chainId, watcherId, watcherKey } = params;

        // Set up heartbeat to ensure we still own the watcher
        const watchdogInterval = this.setupHeartbeat(watcherKey, watcherId, safeAddress, chainId);

        // Store the interval for cleanup
        this.activeWatchdogs.set(watcherKey, watchdogInterval);

        // Set up the event watcher
        const unwatch = publicClient.watchEvent({
            address: safeAddress as `0x${string}`,
            event: parseAbiItem('event ApproveHash(bytes32 hashToApprove, address owner)'),
            onLogs: async (logs: any) => {
                for (const log of logs) {
                    // We'll dispatch to the SafeService for processing these events
                    this.onApproveHashEvent?.(log, publicClient, {
                        email,
                        accountCode,
                        ethAddress,
                        safeAddress,
                        chainId
                    });
                }
            },
            onError: (error: any) => {
                logger.error('Error in event watcher', {
                    error: error instanceof Error ? error.message : 'Unknown error',
                    safeAddress,
                    chainId
                });

                // Clean up on error
                this.cleanupWatcher(watcherKey, watcherId, unwatch, watchdogInterval);
            }
        });

        logger.info('Event watcher setup complete', { safeAddress, chainId });

        // Set up cleanup handler for shutdown
        process.on('beforeExit', () => {
            this.cleanupWatcher(watcherKey, watcherId, unwatch, watchdogInterval);
        });
    }

    private setupHeartbeat(
        watcherKey: string,
        watcherId: string,
        safeAddress: string,
        chainId: number
    ): SafeTimeout {
        return setInterval(async () => {
            try {
                // Check if we still own the watcher
                const currentWatcherId = await this.lock.client.get(watcherKey);
                if (currentWatcherId !== watcherId) {
                    logger.warn('Lost ownership of watcher, shutting down', {
                        safeAddress,
                        chainId,
                        expected: watcherId,
                        actual: currentWatcherId
                    });

                    // Clean up our watcher
                    this.activeWatchdogs.delete(watcherKey);
                    clearInterval(this.activeWatchdogs.get(watcherKey)!);
                    return;
                }
            } catch (error) {
                logger.error('Error in watcher heartbeat', {
                    error: error instanceof Error ? error.message : 'Unknown error',
                    safeAddress,
                    chainId
                });
            }
        }, 30000) as SafeTimeout;
    }

    private async cleanupWatcher(
        watcherKey: string,
        watcherId: string,
        unwatch: () => void,
        interval: SafeTimeout
    ) {
        try {
            // Only delete if we still own it
            const currentWatcherId = await this.lock.client.get(watcherKey);
            if (currentWatcherId === watcherId) {
                await this.lock.client.del(watcherKey);
            }

            unwatch();
            clearInterval(interval);
            this.activeWatchdogs.delete(watcherKey);
            logger.debug('Watcher cleaned up', { watcherKey });
        } catch (error) {
            logger.error('Error cleaning up watcher', {
                error: error instanceof Error ? error.message : 'Unknown error',
                watcherKey
            });
        }
    }

    // Event handler hook - will be set by the SafeService
    onApproveHashEvent?: (
        log: any,
        publicClient: PublicClient,
        params: WatcherParams
    ) => Promise<void>;

    async shutdown() {
        // Clean up all active watchers
        const cleanupPromises = Array.from(this.activeWatchdogs.entries()).map(async ([key, interval]) => {
            try {
                clearInterval(interval);

                // Only delete if we still own it
                const watcherId = await this.lock.client.get(key);
                if (watcherId) {
                    await this.lock.client.del(key);
                }

                this.activeWatchdogs.delete(key);
            } catch (e) {
                logger.error(`Failed to clean up watcher ${key}`, {
                    error: e instanceof Error ? e.message : 'Unknown error'
                });
            }
        });

        await Promise.all(cleanupPromises);
        await this.lock.cleanup();
        logger.info('Safe watchers shutdown complete');
    }
} 