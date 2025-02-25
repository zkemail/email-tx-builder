import express from 'express';
import { config } from 'dotenv';
import accountRoutes from './routes/account.routes';
import { SafeService } from './services/safe.service';

config();

const app = express();
app.use(express.json());

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

// Start the Safe monitor
const safeService = new SafeService();
safeService.start(5000);

// Handle graceful shutdown
process.on('SIGTERM', async () => {
    console.log('SIGTERM received. Shutting down gracefully...');
    await safeService.shutdown();
    process.exit(0);
});

process.on('SIGINT', async () => {
    console.log('SIGINT received. Shutting down gracefully...');
    await safeService.shutdown();
    process.exit(0);
});
