use rust_blockchain_example::{Block, Blockchain, Output, Transaction};

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let chain = Blockchain::new();
    let first_block = gen_first_block(difficulty);
    let last_hash = first_block.hash.clone();

    println!(
        "chain:{:?}, block:{:?}, hash:{:?}",
        chain, first_block, last_hash
    );
}

fn gen_first_block(difficulty: u128) -> Block {
    let mut block = Block::new(
        0,
        0,
        vec![0; 32],
        0,
        vec![Transaction {
            outputs: vec![
                Output {
                    to_addr: "Alice".to_owned(),
                    value: 100,
                },
                Output {
                    to_addr: "Bob".to_owned(),
                    value: 200,
                },
            ],
            inputs: vec![],
        }],
        difficulty,
    );

    block.mine();
    println!("Mined genesis block={:?}, hash={:?}", &block, &block.hash);
    block
}
