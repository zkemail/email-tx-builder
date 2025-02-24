# safe_notification_service

To install dependencies:

```bash
bun install
```

To run the server in development mode:

first create `.env` file according to `.env.example`

then create `src/config/chains.json` file according to `src/config/chains.example.json`
Then run the docker compose file to start the postgres database:

```bash
docker compose up -d
```

Then run the prisma migrations:

```bash
bunx prisma generate
bunx prisma db push
```

Then run the server:

```bash
bun run dev
```

To run the tests:

run migrations for test database:

```bash
DATABASE_URL="postgresql://user:password@localhost:5433/safe_monitor_test" bunx prisma db push
```

run tests:

```bash
bun test
```

