# MPC Garble Interop

This crate provides the circuit generation and optimization layer for the MPC platform, translating high-level function descriptions into optimized boolean circuits.

## Overview

The garble interop layer:
- Parses SFDL (Secure Function Definition Language) input
- Generates optimized boolean circuits
- Provides circuit optimization passes
- Handles circuit validation and verification

## Components

### SFDL Parser

Located in `src/parser/`:
- Grammar definition
- AST generation
- Type checking
- Semantic analysis

### Circuit Generation

Located in `src/circuit/`:
- Boolean circuit construction
- Gate optimization
- Circuit validation
- Size estimation

### Optimization Passes

Located in `src/opt/`:
- Constant propagation
- Dead gate elimination
- Circuit minimization
- XOR optimization

## SFDL Language

The Secure Function Definition Language (SFDL) allows high-level description of computations:

```sfdl
// Example SFDL function
function millionaire_problem(int<32> alice_worth, int<32> bob_worth) -> bool {
    return alice_worth > bob_worth;
}

// Array operations
function sum_array(int<8>[4] values) -> int<8> {
    int<8> sum = 0;
    for (int i = 0; i < 4; i++) {
        sum += values[i];
    }
    return sum;
}
```

## Usage

### Circuit Generation

```rust
use mpc_garble_interop::{Parser, CircuitGenerator};

// Parse SFDL input
let ast = Parser::parse(sfdl_code)?;

// Generate circuit
let circuit = CircuitGenerator::new()
    .optimize(true)
    .generate(ast)?;
```

### Optimization

```rust
use mpc_garble_interop::optimize::{ConstProp, DeadGateElim};

// Apply optimization passes
let circuit = circuit
    .apply_pass(ConstProp::new())
    .apply_pass(DeadGateElim::new());
```

## Circuit Format

The generated circuits follow a standard format:

```rust
struct Circuit {
    gates: Vec<Gate>,
    inputs: Vec<WireRef>,
    outputs: Vec<WireRef>,
}

enum Gate {
    And(WireRef, WireRef),
    Xor(WireRef, WireRef),
    Not(WireRef),
    // ...
}
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
# Run unit tests
cargo test

# Test specific components
cargo test --test parser
cargo test --test circuit
cargo test --test optimize
```

### Benchmarking

```bash
# Run benchmarks
cargo bench
```

## Contributing

When working on the garble interop layer:

1. Add tests for new SFDL features
2. Document optimization passes
3. Maintain backward compatibility
4. Consider circuit size implications

## Circuit Optimization Guidelines

1. Gate Reduction
   - Minimize AND gates
   - Utilize free XOR gates
   - Remove redundant gates

2. Depth Optimization
   - Balance circuit depth
   - Parallelize when possible
   - Consider evaluation cost

3. Memory Usage
   - Optimize wire numbering
   - Minimize temporary storage
   - Consider memory locality

## References

1. Yao, A. C. (1986). How to generate and exchange secrets.
2. Bellare, M., et al. (2012). Foundations of garbled circuits.
3. Zahur, S., et al. (2015). Two halves make a whole: Reducing data transfer in garbled circuits.
