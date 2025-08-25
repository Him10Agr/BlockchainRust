#[derive(Debug, Clone)]
pub struct User{
    address: &'static str,
    balance: f64,
}

impl User {

    pub fn new(address: &'static str, balance: f64) -> Self {
        return Self { address: address, balance: balance };
    }

    pub fn get_address(self: &Self) -> &str {
        return self.address;
    }

    pub fn get_balance(self: &Self) -> f64 {
        return self.balance;
    }
}