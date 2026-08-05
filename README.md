# kms-service

**Production-style Key Management Service (KMS) written in Rust** built with **Axum + Tokio**, designed as a secure, high-performance microservice for cryptographic key operations and secret lifecycle management.

This project demonstrates **enterprise-grade Hexagonal / Clean Architecture** with strong separation of concerns (domain / application / infrastructure), Envelope Encryption, asymmetric key pair generation, structured configuration, observability, and distributed rate limiting.

---

## Highlights

- **Async HTTP API** powered by **Axum (0.8)** + **Tokio**
- **Clean Architecture approach**
  - `domain` defines core contracts (ports), entities, and cryptographic interfaces
  - `application` implements isolated Use Cases (`UnlockSecretUseCase`, etc.)
  - `infrastructure` provides MongoDB / Redis adapters and cryptographic implementations
- **Enterprise-Grade KMS & Security**
  - **Envelope Encryption** using **AES-256-GCM** with a 32-byte Master Key
  - Asymmetrical key pair generation for **Ed25519** (Digital Signatures) and **X25519** (Key Exchange)
  - Memory-safe key handling with `ZeroizeOnDrop`
- **MongoDB integration** with repository pattern (`MongoVaultRepository`, `MongoUserRepository`)
- **Redis integration** (Fred client) for caching and distributed rate limiting
- **Two-level rate limiting**
  - In-memory governor-based limits (`tower-governor`)
  - Redis-backed limiter using **Lua script + EVALSHA optimization**
- **Security middleware**
  - CSP, XSS protection, strict transport security, nosniff, frame deny
- **Structured logging**
  - Console logs + JSON file logging with `tracing` spans and request tracing middleware
- **Graceful shutdown** with configurable timeout
