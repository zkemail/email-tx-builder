import { describe, it, expect, beforeEach, vi } from 'vitest';
import request from 'supertest';
import express from 'express';
import accountRoutes from '../routes/account.routes';
import prisma from '../config/database';
import * as addressCalculator from '../utils/addressCalculator';

// Setup express app
const app = express();
app.use(express.json());
app.use('/api/accounts', accountRoutes);

describe('Account Routes', () => {
    // Clear database before each test
    beforeEach(async () => {
        await prisma.account.deleteMany();
    });

    describe('POST /api/accounts/register', () => {
        it('should reject account creation without safe address', async () => {
            const accountData = {
                email: 'test@example.com',
                accountCode: '0x22a2d51a892f866cf3c6cc4e138ba87a8a5059a1d80dea5b8ee8232034a105b7',
                chainId: 84532
            };

            const response = await request(app)
                .post('/api/accounts/register')
                .send(accountData);

            expect(response.status).toBe(400);
            expect(response.body.error).toBe('Invalid input');
            expect(response.body.details[0].path).toContain('safeAddress');
        });

        it('should create a new account with valid data including safe address', async () => {
            const accountData = {
                email: 'test@example.com',
                accountCode: '0x22a2d51a892f866cf3c6cc4e138ba87a8a5059a1d80dea5b8ee8232034a105b7',
                chainId: 84532,
                safeAddress: '0x1234567890123456789012345678901234567890'
            };

            const response = await request(app)
                .post('/api/accounts/register')
                .send(accountData);

            expect(response.status).toBe(201);
            expect(response.body).toMatchObject({
                email: accountData.email,
                accountCode: accountData.accountCode,
                chainId: accountData.chainId,
                safeAddresses: [accountData.safeAddress]
            });
            expect(response.body.ethAddress).toMatch(/^0x[a-fA-F0-9]{40}$/);
        });

        it('should return 400 for invalid email', async () => {
            const invalidData = {
                email: 'invalid-email',
                accountCode: 'TEST123'
            };

            const response = await request(app)
                .post('/api/accounts/register')
                .send(invalidData);

            expect(response.status).toBe(400);
        });

        it('should return 409 for duplicate eth address', async () => {
            const accountData = {
                email: 'test@example.com',
                accountCode: '0x22a2d51a892f866cf3c6cc4e138ba87a8a5059a1d80dea5b8ee8232034a105b7',
                chainId: 84532,
                safeAddress: '0x1234567890123456789012345678901234567890'
            };

            // Create first account
            await request(app)
                .post('/api/accounts/register')
                .send(accountData);

            // Try to create second account with same eth address
            const response = await request(app)
                .post('/api/accounts/register')
                .send(accountData);

            expect(response.status).toBe(409);
        });

        it('should add new safe address to existing account', async () => {
            const accountData = {
                email: 'test@example.com',
                accountCode: '0x22a2d51a892f866cf3c6cc4e138ba87a8a5059a1d80dea5b8ee8232034a105b7',
                chainId: 84532,
                safeAddress: '0x1234567890123456789012345678901234567890'
            };

            // Create first account
            const firstResponse = await request(app)
                .post('/api/accounts/register')
                .send(accountData);

            expect(firstResponse.status).toBe(201);

            // Add new safe address to same account
            const newSafeAddress = '0x2234567890123456789012345678901234567890';
            const secondResponse = await request(app)
                .post('/api/accounts/register')
                .send({
                    ...accountData,
                    safeAddress: newSafeAddress
                });

            expect(secondResponse.status).toBe(200);
            expect(secondResponse.body.safeAddresses).toHaveLength(2);
            expect(secondResponse.body.safeAddresses).toContain(accountData.safeAddress);
            expect(secondResponse.body.safeAddresses).toContain(newSafeAddress);
        });

        it('should reject duplicate safe address for same account', async () => {
            const accountData = {
                email: 'test@example.com',
                accountCode: '0x22a2d51a892f866cf3c6cc4e138ba87a8a5059a1d80dea5b8ee8232034a105b7',
                chainId: 84532,
                safeAddress: '0x1234567890123456789012345678901234567890'
            };

            // Create first account
            await request(app)
                .post('/api/accounts/register')
                .send(accountData);

            // Try to add same safe address again
            const response = await request(app)
                .post('/api/accounts/register')
                .send(accountData);

            expect(response.status).toBe(409);
            expect(response.body.error).toBe('Safe address already registered for this account');
        });
    });
}); 