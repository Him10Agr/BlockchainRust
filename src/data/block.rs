use std::time::SystemTime;
use sha2::{Sha256, Digest};

use crate::models::transaction::Transaction;
use super::user::User;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub prev_hash: Option<String>,
    pub hash: String,
    pub data: Vec<Transaction>,
    pub nonce: u32,
}

const DIFFICULTY: usize = 4;

impl Block {

    pub fn new(index: u32, prev_hash: Option<String>, data: Vec<Transaction>) -> Self {

        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let hash = Self::calculte_hash(index, timestamp, prev_hash.clone(), data.clone(), 0);

        return Self { index: index, 
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            prev_hash: prev_hash, hash: hash, data: data, nonce: 0 }
    }

    pub fn calculte_hash(index: u32, timestamp: u64, prev_hash: Option<String>, data: Vec<Transaction>, nonce: u32) -> String {

        let prev_hash = prev_hash.unwrap_or(String::from(""));
        let data = format!("{},{},{},{:#?},{}", index, timestamp, prev_hash, data, nonce);
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        return format!("{:x}", result);
    }

    pub fn mine(self: &mut Self) {

        let target = "0".repeat(DIFFICULTY);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            let prev_hash = self.prev_hash.clone();
            let data = self.data.clone();
            self.hash = Block::calculte_hash(self.index, 
                self.timestamp, prev_hash, data, self.nonce);
        }
    }

    pub fn add_txs(self: &mut Self, sender: User, receiver: User, amount: f64) {

        let tx = Transaction::new(sender, receiver, amount);
        self.data.push(tx);
    }
}

