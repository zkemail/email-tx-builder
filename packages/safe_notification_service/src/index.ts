import express from 'express';
import { config } from 'dotenv';
import accountRoutes from './routes/account.routes';
import { SafeMonitorService } from './services/safeMonitor.service';

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
const safeMonitor = new SafeMonitorService();
safeMonitor.start();

// Handle graceful shutdown
process.on('SIGTERM', () => {
    console.log('SIGTERM received. Shutting down gracefully...');
    safeMonitor.stop();
    process.exit(0);
});

process.on('SIGINT', () => {
    console.log('SIGINT received. Shutting down gracefully...');
    safeMonitor.stop();
    process.exit(0);
});
