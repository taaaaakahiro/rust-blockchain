use rust_blockchain_example::{Block, Blockchain};

fn main() {
    let chain = Blockchain::new();
    let block = Block::new();

    println!("chain:{:?}, block:{:?}", chain, block);
}
