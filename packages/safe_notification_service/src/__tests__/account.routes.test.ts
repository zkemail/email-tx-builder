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

    describe('POST /api/accounts', () => {
        it('should create a new account with valid data', async () => {
            const accountData = {
                email: 'thezdev1@gmail.com',
                accountCode: '0x22a2d51a892f866cf3c6cc4e138ba87a8a5059a1d80dea5b8ee8232034a105b7',
                chainId: 84532
            };

            const response = await request(app)
                .post('/api/accounts')
                .send(accountData);

            expect(response.status).toBe(201);
            expect(response.body).toMatchObject({
                email: accountData.email,
                accountCode: accountData.accountCode,
                chainId: accountData.chainId,
                ethAddress: '0xE39796F88Dd07631A7566D4f83A8C229D6F3ca55'
            });
        });

        it('should return 400 for invalid email', async () => {
            const invalidData = {
                email: 'invalid-email',
                accountCode: 'TEST123'
            };

            const response = await request(app)
                .post('/api/accounts')
                .send(invalidData);

            expect(response.status).toBe(400);
        });

        it('should return 409 for duplicate eth address', async () => {
            const accountData = {
                email: 'thezdev1@gmail.com',
                accountCode: '0x22a2d51a892f866cf3c6cc4e138ba87a8a5059a1d80dea5b8ee8232034a105b7',
                chainId: 84532
            };

            // Create first account
            await request(app)
                .post('/api/accounts')
                .send(accountData);

            // Try to create second account with same eth address
            const response = await request(app)
                .post('/api/accounts')
                .send({
                    ...accountData,
                });

            expect(response.status).toBe(409);
        });
    });
}); 