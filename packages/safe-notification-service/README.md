# Safe Notification Service

## Installation

### 1. Install Dependencies
```bash
bun install
```

### 2. Set Up Environment Variables
Create a `.env` file based on `.env.example`.

## Running the Server in Development Mode

### 3. Start the Relay Service
This backend depends on the relay service. If using a local relayer:

1. Run the following command in the root directory to start the IMAP, SMTP service:
   ```bash
   docker compose up -d
   ```
2. Navigate to the `relayer` package directory:
   ```bash
   cd packages/relayer
   ```
3. Start the relayer:
   ```bash
   cargo run
   ```

### 4. Configure Chains
Create `src/config/chains.json` based on `src/config/chains.example.json`.

### 5. Start the Database
Run the following command to start the PostgreSQL database:
```bash
docker compose up -d
```

### 6. Run Prisma Migrations
```bash
bunx prisma generate
bunx prisma db push
```

### 7. Start the Server
```bash
bun run dev
```

## Running Tests

### 1. Set Up Test Database
Run migrations for the test database:
```bash
DATABASE_URL="postgresql://user:password@localhost:5433/safe_monitor_test" bunx prisma db push
```

### 2. Run Tests
```bash
bun test


## sample request
```bash
curl --location 'http://localhost:3030/api/accounts/approve-hash' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "thezdev1@gmail.com",
    "accountCode": "0x22a2d51a892f866cf3c6cc4e138ba87a8a5059a1d80dea5b8ee8232034a105b7",
    "chainId": 11155111,
    "safeAddress": "0x433AeBc31766A0462D7efFaC721A416b2B431096",
    "hashToApprove": "0x323ec421fa9a985aa44400d2308bf0b706b7aff73b115d286dc887333a40a7e1"
}'
```