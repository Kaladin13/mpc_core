# Secure Two-Party Computation Platform

A Rust implementation of Secure Function Evaluation (SFE) using Yao's Garbled Circuits protocol for two-party computation. This project implements secure multi-party computation (MPC) allowing two parties to jointly compute functions without revealing their private inputs.

## Overview

This platform enables two parties to perform joint computations while maintaining the privacy of their inputs. It is based on Yao's Garbled Circuits protocol (1986) and implements modern cryptographic techniques for secure function evaluation.

### Key Features

- Two-party secure computation protocol
- Implementation of Yao's Garbled Circuits
- Network communication layer for distributed computation
- High-level DSL (Domain Specific Language) for defining computations
- Command-line interface for easy interaction
- Docker support for simple deployment

## Project Structure

The project is organized into several Rust crates:

- `mpc_core/`: Core implementation of the MPC protocol and garbled circuits
- `mpc_http_server/`: Server implementation for network communication
- `mpc_http_client/`: Client implementation for network communication
- `mpc_garble_interop/`: Interoperability layer for garbled circuit generation

## Prerequisites

- Rust 1.70 or higher
- Docker (optional, for containerized deployment)
- Network connectivity between participating parties

## Building the Project

1. Clone the repository:
```bash
git clone <repository-url>
cd mpc_core
```

2. Build using Cargo:
```bash
cargo build --release
```

3. Run tests:
```bash
cargo test
```

## Docker Deployment

1. Build the Docker image:
```bash
docker build -t mpc-platform .
```

2. Run the container:
```bash
docker run -p 8080:8080 mpc-platform
```

## Usage

1. Start the server:
```bash
cargo run --bin mpc_http_server
```

2. Run the client (on each party's machine):
```bash
cargo run --bin mpc_http_client
```

3. Follow the CLI prompts to:
   - Input the computation specification in SFDL format
   - Provide private inputs
   - Receive computation results

## Testing

The project includes several test suites:

- Unit tests: `cargo test`
- Integration tests: `cargo test --test '*'`
- Performance benchmarks: `cargo bench`


## Academic References

1. Yao, A. C. (1986). How to generate and exchange secrets. 27th Annual Symposium on Foundations of Computer Science (sfcs 1986).

2. Goldreich, O., Micali, S., & Wigderson, A. (1987). How to play ANY mental game. Proceedings of the nineteenth annual ACM symposium on Theory of computing.

3. Bellare, M., Hoang, V. T., & Rogaway, P. (2012). Foundations of garbled circuits. Proceedings of the 2012 ACM conference on Computer and communications security.

4. Yao, A. C. (1982). Protocols for secure computations. 23rd Annual Symposium on Foundations of Computer Science (sfcs 1982).

5. Lindell, Y., & Pinkas, B. (2009). A proof of security of Yao's protocol for two-party computation. Journal of Cryptology.

