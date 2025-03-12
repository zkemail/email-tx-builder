#!/bin/sh
set -e

# Generate Prisma client
echo "Generating Prisma client..."
bunx prisma generate

# Run database migrations
echo "Running database migrations..."
bunx prisma db push

# Start the application
echo "Starting application..."
bun run start
