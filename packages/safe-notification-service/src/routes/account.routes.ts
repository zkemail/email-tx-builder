import { Router } from 'express';
import { registerAccount } from '../controllers/account.controller';

const router = Router();

router.post('/register', async (req, res) => {
    await registerAccount(req, res);
});

export default router;