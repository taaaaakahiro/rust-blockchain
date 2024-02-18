#[allow(unused_imports)]
use crate::hashable::Hashable;
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    // pub hash: Hash,
    // pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

impl Block {
    pub fn new() -> Self {
        Block {
            //todo
            index: 0,
            timestamp: 0,
            nonce: 0,
            transactions: vec![],
            difficulty: 0,
        }
    }
    // pub fn mine(&mut self) {
    //     //todo
    // }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let bytes = vec![];
        //todo
        bytes
    }
}
