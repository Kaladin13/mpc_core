//! Simulation environment for secure computation under ideal functionality

use crate::{
    states::{Contributor, Evaluator},
    Circuit, Error,
};
use rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;

/// Simulates the local execution of the circuit using a 2-party secure computation protocol.
///
/// The computation is performed using the cryptographic protocol exposed by the
/// [`Contributor`] and [`Evaluator`]. Messages are exchanged using local message queues,
/// simulating execution under ideal network conditions.
pub fn simulate(
    circuit: &Circuit,
    input_contributor: &[bool],
    input_evaluator: &[bool],
) -> Result<Vec<bool>, Error> {
    let mut eval = Evaluator::new(
        circuit.clone(),
        input_evaluator,
        ChaCha20Rng::from_entropy(),
    )?;
    let (mut contrib, mut msg_for_eval) =
        Contributor::new(circuit, input_contributor, ChaCha20Rng::from_entropy())?;

    assert_eq!(contrib.steps(), eval.steps());

    for _ in 0..eval.steps() {
        let (next_state, msg_for_contrib) = eval.run(&msg_for_eval)?;
        eval = next_state;

        let (next_state, reply) = contrib.run(&msg_for_contrib)?;
        contrib = next_state;

        msg_for_eval = reply;
    }
    eval.output(&msg_for_eval)
}
