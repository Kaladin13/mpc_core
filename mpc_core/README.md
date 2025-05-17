# MPC Core Implementation

This crate contains the core implementation of the Two-Party Secure Function Evaluation (SFE) protocol based on Yao's Garbled Circuits.

## Structure

Important files

- `src/circuit.rs`: Boolean circuit representation and manipulation
- `src/ot_base.rs`: Base Oblivious Transfer protocol implementation
- `src/protocol.rs`: Core protocol state machine and logic
- `src/simulator.rs`: Protocol simulation for testing and verification
- `src/states.rs`: State transitions for the two-party protocol

## Protocol Overview

The implementation follows Yao's Garbled Circuits protocol with modern optimizations:

1. Circuit Generation
   - Boolean circuits with AND, XOR, and NOT gates
   - Optimized circuit representation for memory efficiency

2. Garbling Phase
   - Point-and-permute optimization
   - Free-XOR technique
   - Row-reduction optimization

3. Oblivious Transfer
   - Base OT using RSA-based cryptography
   - OT extension for efficiency
   - Delta-based optimization

4. Evaluation
   - Sequential circuit evaluation
   - Constant memory overhead
   - Optimized wire label handling

## Usage

This crate is meant to be used as a library by higher-level applications. Example usage:

```rust
use mpc_core::{Circuit, Gate, Contributor, Evaluator};

// Create a circuit
let circuit = Circuit::new(
    vec![Gate::InContrib, Gate::InEval, Gate::And(0, 1)],
    vec![2],
);

// Initialize parties
let (contributor, msg) = Contributor::new(&circuit, inputs, rng)?;
let evaluator = Evaluator::new(&circuit, inputs, rng)?;

// Run the protocol...
```

## Testing

The crate includes comprehensive test suites:

```bash
# Run unit tests
cargo test

# Run specific test suite
cargo test --test circuit_tests

# Run benchmarks
cargo bench
```

## References

1. Yao, A. C. (1986). How to generate and exchange secrets.
2. Bellare, M., et al. (2012). Foundations of garbled circuits.
3. Zahur, S., et al. (2015). Two halves make a whole: Reducing data transfer in garbled circuits.