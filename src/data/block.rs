use std::time::SystemTime;
use rsa::{Pkcs1v15Sign};
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
    pub sign: Vec<u8>,
    pub validator_addr: Option<String>,
}

const DIFFICULTY: usize = 4;

impl Block {

    pub fn new(index: u32, prev_hash: Option<String>, data: Vec<Transaction>) -> Self {

        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let hash = Self::calculte_hash(index, timestamp, prev_hash.clone(), data.clone(), 0);

        return Self { index: index, 
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            prev_hash: prev_hash, hash: hash, data: data, nonce: 0, sign: Vec::<u8>::new(), validator_addr: None }
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
        let start_time = SystemTime::now();
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            let prev_hash = self.prev_hash.clone();
            let data = self.data.clone();
            self.hash = Block::calculte_hash(self.index, 
                self.timestamp, prev_hash, data, self.nonce);
        }
        let elapsed_time = SystemTime::now().duration_since(start_time).unwrap();
        println!("Blocked mined in {} seconds", elapsed_time.as_secs());
    }

    pub fn sign(self: &mut Self, validator: &User) {
        let private_key = validator.get_private_key();
        let mut hasher = Sha256::new();
        hasher.update(format!("{:#?}{}{:#?}", self.prev_hash, self.hash, self.data));
        let hash = hasher.finalize();
        self.sign = private_key.sign(Pkcs1v15Sign::new::<Sha256>(), &hash).expect("Failed to sign block");
        self.validator_addr = Some(validator.get_address().to_owned());
    }

    pub fn add_txs(self: &mut Self, sender: User, receiver: User, amount: f64) {

        let tx = Transaction::new(sender, receiver, amount);
        self.data.push(tx);
    }
}

