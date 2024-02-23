#[allow(unused_imports)]
use crate::hashable::Hashable;
use crate::transaction::Transaction;
use crate::{difficulty_bytes_as_u128, Hash};

pub fn check_difficulty(hash: &Hash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(hash)
}

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u64,
        prev_block_hash: Hash,
        nonce: u64,
        transactions: Vec<Transaction>,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            transactions,
            difficulty,
        }
    }
    pub fn mine(&mut self) {
        // todo
        // for nonce_attempt in 0..(u64::MAX) {
        //     self.nonce = nonce_attempt;
        //     let hash = self.hash();
        //     if check_difficulty(&hash, self.difficulty) {
        //         self.hash = hash;
        //         return;
        //     }
        // }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let bytes = vec![];
        //todo
        bytes
    }
}
