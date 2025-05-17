# MPC HTTP Server

This crate implements the server-side component of the MPC platform, providing a RESTful API for coordinating secure two-party computations.

## Overview

The HTTP server acts as a coordinator between two parties performing secure computation. It:
- Manages protocol sessions
- Handles message routing between parties
- Provides API endpoints for circuit submission and evaluation
- Manages computation state and results

## Architecture

### Components

- `src/main.rs`: Server entry point and configuration
- `src/routes/`: API endpoint implementations
- `src/handlers/`: Business logic for request handling
- `src/state/`: Server state management
- `src/models/`: Data structures and type definitions

## Configuration

Server configuration is managed through `Rocket.toml`:

```toml
[default]
address = "0.0.0.0"
port = 8080
workers = 16
```

Environment variables:
- `RUST_LOG`: Logging level (info, debug, etc.)
- `SESSION_TIMEOUT`: Session timeout in seconds
- `MAX_CIRCUIT_SIZE`: Maximum allowed circuit size

## Deployment

### Local Development

```bash
cargo run
```

### Docker

```bash
docker build -t mpc-server .
docker run -p 8080:8080 mpc-server
```

### Production

The server includes a `fly.toml` configuration for deployment on Fly.io:

```bash
flyctl deploy
```

## Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test '*'

# Load testing
cargo bench
``