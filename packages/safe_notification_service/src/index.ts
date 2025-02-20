import express from 'express';
import { config } from 'dotenv';
import accountRoutes from './routes/account.routes';

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

// Example of monitoring a Safe
async function monitorSafe(safeAddress: string) {
    // Implementation details will go here
    // You'll want to:
    // 1. Listen for Safe events
    // 2. Store transactions in the database
    // 3. Send email notifications
    // 4. Handle responses
}