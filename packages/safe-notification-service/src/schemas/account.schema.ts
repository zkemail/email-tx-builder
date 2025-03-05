import { z } from 'zod';
import { SUPPORTED_CHAINS } from '../config/chains';

export const RegisterAccountSchema = z.object({
    email: z.string().email(),
    accountCode: z.string().min(1),
    chainId: z.number().refine(
        (val) => val in SUPPORTED_CHAINS,
        { message: 'Unsupported chain ID' }
    ),
    safeAddress: z.string().regex(/^0x[a-fA-F0-9]{40}$/, {
        message: "Safe address must be a valid Ethereum address"
    })
});

export type RegisterAccountInput = z.infer<typeof RegisterAccountSchema>;