import { z } from 'zod';
import { SUPPORTED_CHAINS } from '../config/chains';

export const RegisterAccountSchema = z.object({
    email: z.string().email(),
    accountCode: z.string().min(1),
    chainId: z.number().refine(
        (val) => val in SUPPORTED_CHAINS,
        { message: 'Unsupported chain ID' }
    )
});

export type RegisterAccountInput = z.infer<typeof RegisterAccountSchema>; 