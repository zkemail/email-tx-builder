import { config } from 'dotenv';
import prisma from '../config/database';

// Load environment variables from .env.test if it exists
config({ path: '.env.test' });

// Clear test database before all tests
beforeAll(async () => {
    await prisma.account.deleteMany();
});

// Close Prisma connection after all tests
afterAll(async () => {
    await prisma.$disconnect();
}); 