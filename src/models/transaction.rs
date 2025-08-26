use rsa::Pkcs1v15Sign;
use sha2::{Digest, Sha256};

use crate::data::user::User;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: User,
    pub reciever: User,
    pub amount: f64,
}

impl Transaction {

    pub fn new(sender: User, receiver: User, amount: f64) -> Self {

        return Self { sender: sender, reciever: receiver, amount: amount };
    }

    pub fn valid_tx(self: &Self) -> bool {
        if self.sender.get_balance() >= self.amount {
            let msg = format!("{}{}{}{}", self.sender.get_address(), self.sender.get_balance(),
                self.reciever.get_address(), self.reciever.get_balance());
            let mut hasher = Sha256::new();
            hasher.update(msg);
            let hash = hasher.finalize();
            let signature = self.sender.get_private_key().sign(Pkcs1v15Sign::new::<Sha256>(), &hash).expect("Failed to sign transaction");
            let sign = signature.as_slice();
            return self.reciever.get_public_key().verify(Pkcs1v15Sign::new::<Sha256>(), &hash, sign).is_ok();
        } else {
            return false;
        }
    }
}