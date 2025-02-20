# safe_notification_service

To install dependencies:

```bash
bun install
```

To run the server in development mode:

first create `.env` file according to `.env.example`

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
