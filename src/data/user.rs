use crate::utility::encryption::generate_rsa_key_pair;
use rsa::{RsaPrivateKey, RsaPublicKey};

#[derive(Debug, Clone)]
pub struct User{
    address: &'static str,
    balance: f64,
    private_key: RsaPrivateKey, 
    public_key: RsaPublicKey,
}

impl User {

    pub fn new(address: &'static str, balance: f64) -> Self {
        let (private_key, public_key) = generate_rsa_key_pair();
        return Self { address: address, balance: balance,
            private_key: private_key, public_key:public_key};
    }

    pub fn get_address(self: &Self) -> &str {
        return self.address;
    }

    pub fn get_balance(self: &Self) -> f64 {
        return self.balance;
    }

    pub fn get_public_key(self: &Self) -> RsaPublicKey {
        return self.public_key.clone();
    }

    pub fn get_private_key(self: &Self) -> RsaPrivateKey {
        return self.private_key.clone();
    }


}