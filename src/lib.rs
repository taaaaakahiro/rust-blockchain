pub type Hash = Vec<u8>;
pub type Address = String;

mod block;
pub use crate::block::Block;

mod hashable;
pub use crate::hashable::Hashable;

mod block_chain;
pub use crate::block_chain::Blockchain;

mod transaction;
pub use crate::transaction::{Output, Transaction};

// 16:32, left bit shift 0xf * 8
pub fn difficulty_bytes_as_u128(v: &Vec<u8>) -> u128 {
    ((v[31] as u128) << (0xf * 8))
        | ((v[30] as u128) << (0xe * 8))
        | ((v[29] as u128) << (0xd * 8))
        | ((v[28] as u128) << (0xc * 8))
        | ((v[27] as u128) << (0xb * 8))
        | ((v[26] as u128) << (0xa * 8))
        | ((v[25] as u128) << (0x9 * 8))
        | ((v[24] as u128) << (0x8 * 8))
        | ((v[23] as u128) << (0x7 * 8))
        | ((v[22] as u128) << (0x6 * 8))
        | ((v[21] as u128) << (0x5 * 8))
        | ((v[20] as u128) << (0x4 * 8))
        | ((v[19] as u128) << (0x3 * 8))
        | ((v[18] as u128) << (0x2 * 8))
        | ((v[17] as u128) << (0x1 * 8))
        | ((v[16] as u128) << 0x0)
}
