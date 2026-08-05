# http_server_rs

**Production-style Rust backend** built with **Axum + Tokio**, designed as a clean, modular foundation for real-world APIs.

This project demonstrates **enterprise-grade architecture** with strong separation of concerns (domain / services / infrastructure), structured configuration, observability, security middleware, and scalable rate limiting.

---

## Highlights

- **Async HTTP API** powered by **Axum (0.8)** + **Tokio**
- **Clean architecture approach**
  - `domain` defines core contracts (ports)
  - `services` implement business logic
  - `infrastructure` provides MongoDB / Redis adapters
- **MongoDB integration** with repository layer (User + Vault)
- **Redis integration** (Fred client) for caching and distributed rate limiting
- **Two-level rate limiting**
  - In-memory governor-based limits (`tower-governor`)
  - Redis-backed limiter using **Lua script + EVALSHA optimization**
- **Security middleware**
  - CSP, XSS protection, nosniff, frame deny
- **CORS configuration** driven by typed settings
- **Structured logging**
  - console logs + JSON logs to file
  - `tracing` spans + request tracing middleware
- **Graceful shutdown** with configurable timeout

---

## Quick Start & Environment Setup

### 1. Configure Environment

Create a `.env` file in the root directory and configure your application credentials:

```env
# Database Credentials (used by Rust App & MongoDB Init Script)
DATABASE__USER=zero_day_app_user
DATABASE__PASSWORD=bardzo_bezpieczne_haslo_aplikacji
DATABASE__NAME=zero_day_db

# Root Credentials (used only by MongoDB Container Setup)
MONGO_ROOT_USER=app_admin
MONGO_ROOT_PASSWORD=twoje_bezpieczne_haslo

# Redis Config
REDIS__PASSWORD=devpassword
```

### 2. Run Infrastructure (Docker)

To spin up MongoDB (with automatic user creation) and Redis, run:

```bash
# Start containers in background
docker compose up -d

# (Optional) Verify that the dedicated application user has been created successfully
docker exec -it zero_day_mongo mongosh -u app_admin -p twoje_bezpieczne_haslo --eval "db.getSiblingDB('zero_day_db').getUsers()"

```

### 3. Run the Rust Application

Once the backing services are healthy, start the Axum API server using one of the following methods:

#### Option A: Run via Cargo (CLI)

```bash
cargo run

```

#### Option B: Debug via VS Code (LLDB)

If you are using Visual Studio Code with the **CodeLLDB** extension, you can debug the application directly.

Create or update your `.vscode/launch.json` with the following configuration:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Debug Rust Server",
      "type": "lldb",
      "request": "launch",
      "program": "${workspaceFolder}/target/debug/http_server_rs",
      "args": [],
      "cwd": "${workspaceFolder}",
      "stopOnEntry": false,
      "preLaunchTask": "cargo build"
    }
  ]
}
```

---

## API Endpoints

- `GET /health` — healthcheck
- `POST /auth/login` — authentication entrypoint (stub)
- `POST /vault/unlock` — decrypts stored encrypted payload and returns decrypted CV

---

## Security / Crypto

Vault data is encrypted using:

- **AES-256-GCM**
- key derivation via **Argon2**
- Base64 encoding for payload storage (`ciphertext`, `salt`, `nonce`)

---

## Project Structure

```bash
src/
├── config/          # typed settings + env support
├── domain/          # entities + repository ports + crypto contracts
├── services/        # use-cases (user + vault)
├── infrastructure/  # MongoDB + Redis adapters, Lua scripts, serialization
├── server/          # router, middleware, extractors, graceful shutdown
├── handlers/        # HTTP handlers (auth, vault, health)
└── errors/          # centralized AppError + API error mapping

```

---

## License

Apache-2.0
