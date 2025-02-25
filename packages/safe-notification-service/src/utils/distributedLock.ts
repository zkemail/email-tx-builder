import Redis from 'ioredis';
import { v4 as uuidv4 } from 'uuid';
import logger from './logger';

export class DistributedLock {
    public client: Redis;
    private locks: Map<string, string> = new Map();

    constructor(redisUrl: string) {
        this.client = new Redis(redisUrl);
        this.client.on('error', (err) => {
            logger.error('Redis error', { error: err.message });
        });
    }

    /**
     * Gets the lock ID for a resource if we own it
     * @param resource The resource identifier
     * @returns The lock ID if we own it, undefined otherwise
     */
    getLockId(resource: string): string | undefined {
        return this.locks.get(resource);
    }

    /**
     * Acquires a distributed lock using Redis
     * @param resource The resource identifier to lock
     * @param ttl Time-to-live in milliseconds for the lock (0 means no expiry)
     * @returns Promise<boolean> true if lock was acquired, false otherwise
     */
    async acquireLock(resource: string, ttl: number = 0): Promise<boolean> {
        const lockId = uuidv4();

        // If ttl is 0, don't set an expiry
        let acquired: string | null;
        if (ttl > 0) {
            acquired = await this.client.set(resource, lockId, 'PX', ttl, 'NX');
        } else {
            acquired = await this.client.set(resource, lockId, 'NX');
        }

        if (acquired === 'OK') {
            this.locks.set(resource, lockId);
            logger.debug('Lock acquired', { resource, lockId, ttl });
            return true;
        }

        logger.debug('Failed to acquire lock', { resource });
        return false;
    }

    /**
     * Releases a previously acquired lock
     * @param resource The resource identifier to unlock
     * @returns Promise<boolean> true if lock was released, false otherwise
     */
    async releaseLock(resource: string): Promise<boolean> {
        const lockId = this.locks.get(resource);
        if (!lockId) {
            logger.debug('No lock found to release', { resource });
            return false;
        }

        // Using Lua script to ensure we only delete the key if it matches our lock ID
        const script = `
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        `;

        const result = await this.client.eval(script, 1, resource, lockId);

        if (result === 1) {
            this.locks.delete(resource);
            logger.debug('Lock released', { resource, lockId });
            return true;
        }

        logger.warn('Failed to release lock', { resource, lockId });
        return false;
    }

    /**
     * Cleans up Redis connection
     */
    async cleanup(): Promise<void> {
        await this.client.quit();
        logger.info('Redis connection closed');
    }
}