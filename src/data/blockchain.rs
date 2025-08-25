use super::block::Block;
use crate::models::transaction::Transaction;
use super::user::User;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub validators: Vec<User>,
}

impl Blockchain {

    pub fn new() -> Self {
        return Self { chain: Vec::<Block>::new(),
                      validators: Vec::<User>::new()};
    }

    pub fn add_block(self: &mut Self, data: Vec<Transaction>) {
        
        let index = (self.chain.len() + 1) as u32;
        let prev_hash = Some(self.chain.last().map_or(String::new(), 
                                    |block| return block.hash.clone()));
        let mut block = Block::new(index, prev_hash, data);
        block.mine();
        let validator = self.select_validator_PoS();
        //block.sign(validator);
        self.chain.push(block);
    }

    pub fn select_validator_PoS(self: &Self) -> &User {

        let total_balance = self.validators.iter().map(|u| u.get_balance()).sum::<f64>();
        let mut cummulative_balance = 0.0 as f64;
        let random_balance = rand::random::<f64>() * total_balance;

        for user in &self.validators {
            cummulative_balance += user.get_balance();
            if cummulative_balance >= random_balance {
                return user;
            }
        }
        return &self.validators[0];
    }
}