use crate::block::Block;
use crate::block_chain::Blockchain;

mod block;
mod block_chain;
mod hashable;
mod transaction;

fn main() {
    let chain = Blockchain::new();
    let block = Block::new();

    println!("chain:{:?}, block:{:?}", chain, block);
}
