//! HTTP server for secure multi-party computation.
//!
//! This crate provides an HTTP server acting as the `contributor` and running the secure computation
//! engine. A connecting HTTP client is expected to act as the `evaluator`.
//!
//! This crate can be used as either a library or a binary.
//!
//! As a library, it provides a [`build`] function, which can be used to construct a server with
//! custom logic for choosing its input.
//!
//! In order to use this crate as a binary, the crate must be compiled with the `bin` feature. The
//! server binary supports two modes of execution:
//!
//! 1. Without configuration: Acts as a simple 'echo server' and expects the contributor's input 
//!    to be supplied by the client (as plaintext metadata).
//!
//! 2. With static configuration: Provided during server startup, as `config.json` or
//!    `config.toml`, describing which function and contributor input to use based on the
//!    plaintext metadata supplied by the client. These must be stored in the directory from which the
//!    server is started. The directory must also contain a file named `program.rs`
//!    with the program to run on the secure computation engine.
//!
//! As the server is based on the [Rocket](https://rocket.rs) framework, it is possible to
//! configure it according to the official [Rocket
//! documentation](https://rocket.rs/v0.5-rc/guide/configuration/#configuration).
//!
//! Example configuration through env vars:
//!
//! ```sh
//! # make the server listen at port 8080
//! ROCKET_PORT=8080 mpc_server
//!
//! # listen at 127.0.0.1 for HTTP requests
//! ROCKET_ADDRESS=127.0.0.1 mpc_server
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

use engine::{stage, Cors};
use rocket::{Build, Rocket};
pub use types::{HandleMpcRequestFn, MpcRequest, MpcSession};

#[macro_use]
extern crate rocket;

mod engine;
mod msg_queue;
mod requests;
mod responses;
mod state;
mod types;

#[cfg(test)]
mod tests;

/// Starts a secure computation server, responding to requests using the specified custom handler logic.
pub fn build(handler: HandleMpcRequestFn) -> Rocket<Build> {
    rocket::build().attach(stage(handler)).attach(Cors)
}
