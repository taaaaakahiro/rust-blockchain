pub type Hash = Vec<u8>;
pub type Address = String;

mod block;
pub use crate::block::Block;

mod hashable;
pub use crate::hashable::Hashable;

mod block_chain;
pub use crate::block_chain::Blockchain;

mod transaction;
pub use crate::transaction::Output;
pub use crate::transaction::Transaction;
