use rocket::serde::{Deserialize, Serialize};
use mpc_core::CircuitBlake3Hash;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewSession {
    pub plaintext_metadata: String,
    pub program: String,
    pub function: String,
    pub circuit_hash: CircuitBlake3Hash,
    pub client_version: String,
}
