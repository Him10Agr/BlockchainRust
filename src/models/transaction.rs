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
        return self.sender.get_balance() >= self.amount;
    }
}