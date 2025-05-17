# Mpc HTTP Client

This crate implements the client-side component of the MPC platform, providing a command-line interface and web client for participating in secure two-party computations.

## Overview

The HTTP client provides:
- Command-line interface for computation setup
- Web interface for interactive sessions
- Protocol message handling and state management
- Input/output handling and formatting

## Components

### Command-line Interface

Located in `src/cli/`:
- Session management
- Circuit input/output
- Configuration handling
- Logging and debugging

## Usage

### Command-line Mode

```bash
# Start new computation
mpc_http_client new --circuit example.circuit

# Submit input
mpc_http_client input --session <id> --value <input>

# Check status
mpc_http_client status --session <id>

# Get results
mpc_http_client result --session <id>
```
## Configuration

The client can be configured through:

1. Command-line arguments
2. Environment variables
3. Configuration file (`config.toml`)

Example configuration:
```toml
[server]
url = "http://localhost:8080"
timeout = 30

[client]
log_level = "info"
max_retries = 3
```

## Development

### Setup

```bash
# Install dependencies
npm install  # for web interface
cargo build

### Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test '*'

# Test web interface
npm test
```

### Benchmarks

Located in `benches/`:
```bash
cargo bench
```