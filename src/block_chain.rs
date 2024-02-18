use crate::block::Block;
use crate::Hash;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub output_hashes: HashSet<Hash>,
    pub unspent_outputs: HashSet<Hash>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            output_hashes: HashSet::new(),
            unspent_outputs: HashSet::new(),
        }
    }
}
