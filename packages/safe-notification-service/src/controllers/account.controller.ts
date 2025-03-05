import type { Request, Response } from 'express';
import { RegisterAccountSchema } from '../schemas/account.schema';
import prisma from '../config/database';
import { z } from 'zod';
import { calculateEthAddress } from '../utils/addressCalculator';
import { SUPPORTED_CHAINS } from '../config/chains';

export const registerAccount = async (req: Request, res: Response) => {
    try {
        // Validate input using zod schema
        const { email, accountCode, chainId, safeAddress } = RegisterAccountSchema.parse(req.body);

        // Check if account already exists with this combination
        const existingAccount = await prisma.account.findFirst({
            where: {
                email,
                accountCode,
                chainId
            }
        });

        if (existingAccount) {
            // Check if safe address already exists for this account
            if (existingAccount.safeAddresses.includes(safeAddress)) {
                return res.status(409).json({
                    error: 'Safe address already registered for this account'
                });
            }

            // Add new safe address to existing account
            const updatedAccount = await prisma.account.update({
                where: { id: existingAccount.id },
                data: {
                    safeAddresses: [...existingAccount.safeAddresses, safeAddress]
                }
            });

            return res.status(200).json(updatedAccount);
        }

        // Validate chain ID
        if (!SUPPORTED_CHAINS[chainId]) {
            return res.status(400).json({
                error: 'Invalid chain ID',
                supportedChains: Object.keys(SUPPORTED_CHAINS)
            });
        }

        // Calculate ETH address
        const ethAddress = await calculateEthAddress(accountCode, email, chainId);

        // Register new account in database and send response
        const account = await prisma.account.create({
            data: {
                email,
                accountCode,
                ethAddress,
                chainId,
                safeAddresses: [safeAddress]
            }
        });

        res.status(201).json(account);
    } catch (error) {
        if (error instanceof z.ZodError) {
            return res.status(400).json({
                error: 'Invalid input',
                details: error.errors
            });
        }
        console.error('Account registration error:', error);
        res.status(500).json({ error: 'Internal server error' });
    }
}