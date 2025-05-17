use crate::{
    ot_base::message::Init as BaseOTInit,
    ot_base::{OtMessage, Receiver as BaseReceiver, Sender as BaseSender},
    types::{Delta, KeyType, MacType, K},
};
use rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub(crate) const BLOCK_SIZE: usize = K;

pub(crate) mod message {
    use serde::{Deserialize, Serialize};

    use crate::{ot_base::message::Init, ot_base::message::InitReply, Error};

    #[derive(Debug, Clone, PartialEq)]
    pub struct OtInit(pub(super) Box<[Init; super::K]>);

    #[derive(Debug, Clone, PartialEq)]
    pub struct OtInitReply(pub(super) Box<[InitReply; super::K]>);

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SerializedOtInit(Vec<u8>);

    impl OtInit {
        pub fn serialize(&self) -> SerializedOtInit {
            let mut buffer = Vec::with_capacity(32 * super::K);
            for init in self.0.iter() {
                init.serialize_to_buffer(&mut buffer);
            }
            SerializedOtInit(buffer)
        }
    }

    impl SerializedOtInit {
        pub fn deserialize(&self) -> Result<OtInit, Error> {
            let mut buffer = self.0.iter();
            let mut init = Box::new([Init::default(); super::K]);
            for init in init.iter_mut().take(super::K) {
                *init = Init::deserialize_from_buffer(&mut buffer)?;
            }
            Ok(OtInit(init))
        }
    }

    impl OtInitReply {
        pub fn serialize(&self) -> Vec<u8> {
            let mut buffer = Vec::with_capacity(crate::ot_base::MSG_LEN * 2 * super::K);
            for init_reply in self.0.iter() {
                init_reply.serialize_to_buffer(&mut buffer);
            }
            buffer
        }

        pub fn deserialize(buffer: Vec<u8>) -> Result<Self, Error> {
            if buffer.len() != crate::ot_base::MSG_LEN * 2 * super::K {
                return Err(Error::OtInitDeserializationError);
            }

            let mut buffer = buffer.iter();
            let mut init_reply =
                Box::new([crate::ot_base::message::InitReply::default(); super::K]);

            for init in init_reply.iter_mut().take(super::K) {
                *init = crate::ot_base::message::InitReply::deserialize_from_buffer(&mut buffer)?;
            }

            Ok(OtInitReply(init_reply))
        }
    }
}

#[derive(Clone)]
pub(crate) struct ReceiverInitializer {
    senders: Box<[BaseSender; K]>,
    ot_messages: Box<[[OtMessage; 2]; K]>,
}

#[derive(Clone)]
pub(crate) struct SenderInitializer {
    delta: Delta,
    receivers: Box<[BaseReceiver; K]>,
}

#[derive(Debug, Clone)]
pub(crate) struct LeakyOtReceiver {
    otg0: Box<[ChaCha20Rng; K]>,
    otg1: Box<[ChaCha20Rng; K]>,
}

#[derive(Debug, Clone)]
pub(crate) struct LeakyOtSender {
    delta: Delta,
    otg: Box<[ChaCha20Rng; K]>,
}

impl ReceiverInitializer {
    pub(crate) fn init(rng: &mut ChaCha20Rng) -> (Self, message::OtInit) {
        let senders = Box::new([(); K].map(|_| BaseSender::new(rng)));
        let mut idxs = [0; K];
        for (i, idx) in idxs.iter_mut().enumerate().take(K) {
            *idx = i;
        }
        let msgs = Box::new(idxs.map(|i| BaseSender::init_message(&senders[i])));

        let ot_messages = Box::new({
            idxs.map(|_| {
                let mut ot_messages = [OtMessage::default(); 2];
                rng.fill_bytes(&mut ot_messages[0]);
                rng.fill_bytes(&mut ot_messages[1]);
                ot_messages
            })
        });

        let s = Self {
            senders,
            ot_messages,
        };

        (s, message::OtInit(msgs))
    }

    pub(crate) fn recv(&self, m: &message::OtInit) -> (LeakyOtReceiver, message::OtInitReply) {
        let mut idxs = [0; K];
        for (i, idx) in idxs.iter_mut().enumerate().take(K) {
            *idx = i;
        }

        let replies =
            Box::new(idxs.map(|idx| {
                BaseSender::send(&self.senders[idx], &m.0[idx], &self.ot_messages[idx])
            }));

        let otg0: Box<[ChaCha20Rng; K]> =
            Box::new(idxs.map(|idx| ChaCha20Rng::from_seed(self.ot_messages[idx][0])));
        let otg1: Box<[ChaCha20Rng; K]> =
            Box::new(idxs.map(|idx| ChaCha20Rng::from_seed(self.ot_messages[idx][1])));

        (
            LeakyOtReceiver { otg0, otg1 },
            message::OtInitReply(replies),
        )
    }
}

impl SenderInitializer {
    pub(crate) fn init(
        rng: &mut ChaCha20Rng,
        delta: Delta,
        m: &message::OtInit,
    ) -> (Self, message::OtInit) {
        let mut idxs: [usize; K] = [0; K];
        for (i, idx) in idxs.iter_mut().enumerate().take(K) {
            *idx = i;
        }

        let mut msgs: Box<[BaseOTInit; K]> = Box::new([BaseOTInit::default(); K]);
        let receivers: Box<[BaseReceiver; K]> = Box::new(idxs.map(|i| {
            let chosen = (delta.0 & (1 << i)) != 0;
            let (msg, r) = BaseReceiver::init(rng, &m.0[i], chosen);
            msgs[i] = msg;
            r
        }));

        (Self { delta, receivers }, message::OtInit(msgs))
    }

    pub(crate) fn recv(self, m: &message::OtInitReply) -> LeakyOtSender {
        let mut idxs = [0; K];
        for (i, idx) in idxs.iter_mut().enumerate().take(K) {
            *idx = i;
        }

        let mut idx = 0;
        let otg = Box::new(self.receivers.map(|r| {
            let seed = r.recv(m.0[idx]);
            idx += 1;
            ChaCha20Rng::from_seed(seed)
        }));

        LeakyOtSender {
            delta: self.delta,
            otg,
        }
    }
}

impl LeakyOtSender {
    pub(crate) fn send(&mut self, ot_rx: &[MacType], keys_out: &mut [MacType]) {
        let mut q_i = [KeyType(0); BLOCK_SIZE];
        for (i, q_i) in q_i.iter_mut().enumerate() {
            let k = (self.otg[i].next_u64() as u128 | ((self.otg[i].next_u64() as u128) << 64))
                ^ if (self.delta.0 & (1 << i)) != 0 {
                    ot_rx[i].0
                } else {
                    0
                };
            *q_i = KeyType(k)
        }

        matrix_transpose(keys_out, &q_i);
    }
}

impl LeakyOtReceiver {
    pub(crate) fn new_batch(
        &mut self,
        random_bits: u128,
        macs_out: &mut [MacType],
        ot_out: &mut [MacType],
    ) {
        assert!(ot_out.len() >= BLOCK_SIZE);
        assert!(macs_out.len() >= BLOCK_SIZE);

        let mut t_i: [KeyType; BLOCK_SIZE] = [KeyType(0); BLOCK_SIZE];
        for (i, t_i) in t_i.iter_mut().enumerate() {
            *t_i = KeyType(self.otg0[i].next_u64() as u128 | ((self.otg0[i].next_u64() as u128) << 64));
        }

        for i in 0..BLOCK_SIZE {
            ot_out[i] = MacType(t_i[i].0 ^ (self.otg1[i].next_u64() as u128 | ((self.otg1[i].next_u64() as u128) << 64)) ^ random_bits);
        }

        matrix_transpose(macs_out, &t_i);
    }
}

#[inline]
fn matrix_transpose(macs_out: &mut [MacType], t_i: &[KeyType]) {
    for (i, mac_out) in macs_out.iter_mut().enumerate().take(BLOCK_SIZE) {
        *mac_out = transpose_column(t_i, 1 << i);
    }
}

#[inline]
fn transpose_column(t_i: &[KeyType], test_bit: u128) -> MacType {
    let mut key = 0;

    for (i, t_i) in t_i.iter().enumerate().take(BLOCK_SIZE) {
        let bit_from_ot = u128::from((t_i.0 & test_bit) != 0);
        key |= bit_from_ot << i;
    }
    MacType(key)
}

#[test]
fn test_serialization() {
    use rand_core::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    let mut rng_send = ChaCha20Rng::from_seed([42; 32]);
    let delta = Delta(rng_send.next_u64() as u128 | ((rng_send.next_u64() as u128) << 64));

    let (r, r_msg) = ReceiverInitializer::init(&mut rng_send);
    let (_, s_msg) = SenderInitializer::init(&mut rng_send, delta, &r_msg);
    let (_, reply) = r.recv(&s_msg);

    assert_eq!(r_msg, r_msg.serialize().deserialize().unwrap());
    assert_eq!(s_msg, s_msg.serialize().deserialize().unwrap());
    assert_eq!(
        reply,
        message::OtInitReply::deserialize(reply.serialize()).unwrap()
    );
}
