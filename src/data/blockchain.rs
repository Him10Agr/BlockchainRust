use super::block::Block;
use crate::models::transaction::Transaction;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {

    pub fn new() -> Self {
        return Self { chain: Vec::<Block>::new()};
    }

    pub fn add_block(self: &mut Self, data: Vec<Transaction>) {
        
        let index = (self.chain.len() + 1) as u32;
        let prev_hash = Some(self.chain.last().map_or(String::new(), 
                                    |block| return block.hash.clone()));
        let mut block = Block::new(index, prev_hash, data);
        block.mine();
        self.chain.push(block);
    }

}