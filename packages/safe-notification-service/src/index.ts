import express from 'express';
import { config } from 'dotenv';
import cors from 'cors';
import accountRoutes from './routes/account.routes';
import { SafeNotificationService } from './services/safe-notification.service';
import SafeApiKit from '@safe-global/api-kit';
import { OnChainConfirmTransactionService } from './services/on-chain-confirm-transaction.service';
import { SUPPORTED_CHAINS } from './config/chains';

config();

const app = express();
app.use(express.json());
app.use(cors({
    origin: process.env.CORS_ORIGIN,
    methods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
    allowedHeaders: ['Content-Type', 'Authorization']
}));

// Basic health check endpoint
app.get('/health', (req, res) => {
    res.json({ status: 'ok' });
});

// Routes
app.use('/api/accounts', accountRoutes);

// Start the server
const port = process.env.PORT || 3000;
app.listen(port, () => {
    console.log(`Server running on port ${port}`);
});

const safeNotificationService = new SafeNotificationService();
safeNotificationService.startPolling(5);

// Handle graceful shutdown
process.on('SIGTERM', async () => {
    console.log('SIGTERM received. Shutting down gracefully...');
    await safeNotificationService.shutdown();
    process.exit(0);
});

process.on('SIGINT', async () => {
    console.log('SIGINT received. Shutting down gracefully...');
    await safeNotificationService.shutdown();
    process.exit(0);
});
