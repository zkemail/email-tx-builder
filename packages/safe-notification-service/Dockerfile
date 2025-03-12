# Use the official Bun image as base
FROM oven/bun:1 AS build

WORKDIR /app

# Install build dependencies required for native modules
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy package.json and install dependencies
COPY package.json .
# Use --ignore-scripts to skip native module compilation
RUN bun install --ignore-scripts --no-save

# Copy app source code
COPY . .

# Generate Prisma client
RUN bunx prisma generate

# Use a smaller base image for the production stage
FROM oven/bun:1-slim

WORKDIR /app

# Copy built app from the previous stage
COPY --from=build /app/node_modules /app/node_modules
COPY --from=build /app/src /app/src
COPY --from=build /app/package.json /app/package.json
COPY --from=build /app/prisma /app/prisma
COPY docker-entrypoint.sh /app/

# Make sure we have the prisma directory with the generated client
COPY --from=build /app/node_modules/.prisma /app/node_modules/.prisma

# Make entrypoint script executable - do this BEFORE changing user
RUN chmod +x /app/docker-entrypoint.sh

# Create a non-root user and change ownership
RUN adduser --disabled-password --gecos "" appuser && \
    chown -R appuser:appuser /app

USER appuser

# Set environment variables
ENV NODE_ENV=production

# Expose the service's port
EXPOSE 3030

# Set the entrypoint script to run when container starts
ENTRYPOINT ["/app/docker-entrypoint.sh"]
