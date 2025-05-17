//! Implements a secure coin tossing protocol between two parties.
//!
//! This protocol allows 2 parties to generate the same random values that can be used
//! to seed their RNGs with the same seed.
//!
//! Protocol steps:
//! 1. Initialize with [`init`] to get a commitment message
//! 2. Use [`serialize`] on the coin share
//! 3. Finish with [`finish`] using the other party's commitment and share messages

use crate::Error;
use serde::{Serialize, Deserialize};

/// Number of bits for a coin.
pub(crate) const COIN_LEN: usize = 32;
/// Number of bits for a commitment.
const HASH_LEN: usize = blake3::OUT_LEN;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct CoinShare([u8; COIN_LEN]);

/// Result of the coin tossing protocol.
pub(crate) type CoinResult = [u8; COIN_LEN];

/// Creates a new coinshare and a message to be shared with another party.
pub(crate) fn init(coin: [u8; COIN_LEN]) -> Result<(CoinShare, Vec<u8>), Error> {
    let hash = hash_coinshare(&coin);
    let msg = bincode::serialize(&hash)?;
    let coin_share = CoinShare(coin);
    Ok((coin_share, msg))
}

/// Serializes a CoinShare to be disclosed to another party at the 2nd protocol step.
pub(crate) fn serialize(cs: &CoinShare) -> Result<Vec<u8>, Error> {
    let msg = bincode::serialize(&cs.0)?;
    Ok(msg)
}

/// Verifies the upstream coinshare and returns the resulting coin.
pub(crate) fn finish(
    coin_share: CoinShare,
    upstream_hash_msg: Vec<u8>,
    upstream_coin: Vec<u8>,
) -> Result<CoinResult, Error> {
    let upstream_hash: [u8; HASH_LEN] = bincode::deserialize(&upstream_hash_msg)?;
    let upstream_coin: [u8; COIN_LEN] = bincode::deserialize(&upstream_coin)?;

    if upstream_hash != hash_coinshare(&upstream_coin) {
        return Err(Error::MacError);
    }

    Ok(xor(coin_share.0, upstream_coin))
}

fn hash_coinshare(s: &[u8; COIN_LEN]) -> [u8; HASH_LEN] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(s);
    let mut output_reader = hasher.finalize_xof();
    let mut result = [0u8; HASH_LEN];
    output_reader.fill(&mut result);

    result
}

fn xor(lhs: [u8; COIN_LEN], rhs: [u8; COIN_LEN]) -> CoinResult {
    let mut result = [0u8; COIN_LEN];
    for i in 0..COIN_LEN {
        result[i] = lhs[i] ^ rhs[i];
    }

    result
}

#[test]
fn test_coinshare() {
    use rand_core::RngCore;
    use rand_chacha::ChaCha20Rng;
    use rand_core::SeedableRng;

    let test_val = (ChaCha20Rng::from_entropy().next_u32() % 255) as u8;
    let coin1 = [test_val; COIN_LEN];
    let coin2 = [!test_val; COIN_LEN];
    let expected = [255u8; COIN_LEN];

    let (coin_share1, commitment_msg1) = init(coin1).unwrap();
    let coin_msg1 = serialize(&coin_share1).unwrap();

    let (coin_share2, commitment_msg2) = init(coin2).unwrap();
    let coin_msg2 = serialize(&coin_share2).unwrap();

    assert_eq!(
        expected,
        finish(coin_share1, commitment_msg2, coin_msg2).unwrap()
    );
    assert_eq!(
        expected,
        finish(coin_share2, commitment_msg1, coin_msg1).unwrap()
    );
}

#[test]
fn test_coinshare_fail() {
    use rand_core::RngCore;
    use rand_chacha::ChaCha20Rng;
    use rand_core::SeedableRng;
    let mut rng = ChaCha20Rng::from_entropy();
    let mut coin1: [u8; COIN_LEN] = Default::default();
    let mut coin2: [u8; COIN_LEN] = Default::default();
    rng.fill_bytes(&mut coin1[0..]);
    rng.fill_bytes(&mut coin2[0..]);
    let coin1 = coin1;
    let coin2 = coin2;

    let corruption_index = (rng.next_u32() as usize) % (COIN_LEN * 8);

    let (coin_share1, _) = init(coin1).unwrap();
    let (coin_share2_ok, commitment_msg2_ok) = init(coin2.clone()).unwrap();
    let coin_msg2_ok = serialize(&coin_share2_ok).unwrap();

    let mut coin2 = coin2.clone();
    coin2[corruption_index / 8] ^= 1 << (corruption_index % 8);

    let (coin_share2_nok, commitment_msg2_nok) = init(coin2).unwrap();
    let coin_msg2_nok = serialize(&coin_share2_nok).unwrap();

    assert_eq!(
        Err(Error::MacError),
        finish(
            coin_share1.clone(),
            commitment_msg2_nok,
            coin_msg2_ok.clone()
        )
    );

    assert_eq!(
        Err(Error::MacError),
        finish(
            coin_share1.clone(),
            commitment_msg2_ok,
            coin_msg2_nok.clone()
        )
    );
}
