import { Router } from 'express';
import { registerAccount, approveHashManually } from '../controllers/account.controller';

const router = Router();

router.post('/register', async (req, res) => {
    await registerAccount(req, res);
});

router.post('/approve-hash', async (req, res) => {
    await approveHashManually(req, res);
});

export default router;