//! Implementation of secure AND computation sub-protocols.
use crate::{
    hash::hash,
    types::{Delta, KeyType, MacType, K},
};
use rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub(crate) type AndHashes = [[MacType; 2]];

/// Computes hashes for each secret value to be sent to the other party.
///
/// Parameters:
/// - `keys[i]` is the key for authenticated bit at index `i`
/// - `random_bits` are the random values used in the protocol
/// - `delta` is the local delta value
pub(crate) fn compute_leaky_and_hashes(
    out: &mut AndHashes,
    delta: &Delta,
    random_bits: u128,
    authenticated_bits_y: u128,
    keys: &[KeyType],
) {
    assert!(keys.len() >= K);
    assert!(out.len() >= K);

    for i in 0..K {
        let random_bit = u128::from(random_bits & (1 << i) != 0);
        let y_bit = u128::from((authenticated_bits_y & (1 << i)) != 0);
        out[i][0] = MacType(hash(MacType(keys[i].0)).0 ^ random_bit);
        out[i][1] = MacType(hash(delta.xor(MacType(keys[i].0))).0 ^ random_bit ^ y_bit);
    }
}

/// Derives AND shares from the received hashes.
///
/// Takes K-many `and_hashes` from the other party which were computed through
/// [compute_leaky_and_hashes] and outputs K-many shares.
pub(crate) fn derive_and_shares(
    random_bits: u128,
    authenticated_bits: u128,
    macs: &[MacType],
    and_hashes: &AndHashes,
) -> MacType {
    assert!(macs.len() >= K);
    assert!(and_hashes.len() >= K);

    let mut result = 0;

    for i in 0..K {
        let idx = usize::from((authenticated_bits & (1 << i)) != 0);
        let is_set = (and_hashes[i][idx].0 ^ hash(macs[i]).0) != 0;
        result |= (u128::from(is_set)) << i;
    }

    MacType(result ^ random_bits)
}

#[test]
fn test_leaky_and() {
    use rand_core::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    let mut rng = ChaCha20Rng::from_entropy();
    let x = rng.next_u64() as u128 | ((rng.next_u64() as u128) << 64);
    let y = rng.next_u64() as u128 | ((rng.next_u64() as u128) << 64);
    let delta = Delta(rng.next_u64() as u128 | ((rng.next_u64() as u128) << 64));

    let mut keys = [KeyType(0); K];
    let mut macs = [MacType(0); K];
    for i in 0..K {
        keys[i] = KeyType(rng.next_u64() as u128 | ((rng.next_u64() as u128) << 64));
        macs[i] = MacType(rng.next_u64() as u128 | ((rng.next_u64() as u128) << 64));
    }

    let mut hashes = [[MacType(0), MacType(0)]; K];
    compute_leaky_and_hashes(&mut hashes, &delta, x, y, &keys);

    let result = derive_and_shares(x, y, &macs, &hashes);
    assert_eq!(result.0, x & y);
}

/// Generates K-many authenticated bits for testing.
#[cfg(test)]
fn gen_abits() -> (
    Delta,
    KeyType,
    [KeyType; 128],
    [MacType; 128],
    Delta,
    KeyType,
    [KeyType; 128],
    [MacType; 128],
) {
    use rand::{random, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    let mut rng = ChaCha20Rng::from_entropy();
    let delta_a = Delta::gen_random(&mut rng);
    let delta_b: Delta = Delta::gen_random(&mut rng);

    let bits_a = KeyType(random());
    let bits_b = KeyType(random());

    let mut keys_a = [KeyType(0); 128];
    for k in keys_a.iter_mut() {
        *k = KeyType(random());
    }
    let mut keys_b = [KeyType(0); 128];
    for k in keys_b.iter_mut() {
        *k = KeyType(random());
    }
    let mut macs_a = [MacType(0); 128];
    for (i, m) in macs_a.iter_mut().enumerate() {
        *m = if bits_a.0 & 1 << i != 0 {
            delta_b.xor(MacType(keys_b[i].0))
        } else {
            MacType(keys_b[i].0)
        };
    }
    let mut macs_b = [MacType(0); 128];
    for (i, m) in macs_b.iter_mut().enumerate() {
        *m = if bits_b.0 & 1 << i != 0 {
            delta_a.xor(MacType(keys_a[i].0))
        } else {
            MacType(keys_a[i].0)
        };
    }
    (
        delta_a, bits_a, keys_a, macs_a, delta_b, bits_b, keys_b, macs_b,
    )
}
