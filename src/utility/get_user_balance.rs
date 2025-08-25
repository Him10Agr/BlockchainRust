use crate::data::{blockchain::Blockchain, user::User};

pub fn get_user_balance(blockchain: &Blockchain, user: User) -> f64 {

    let user_add = user.get_address();
    return blockchain.chain.iter().flat_map(|block| &block.data).
        filter(|tx| tx.sender.get_address() == user_add).
        fold(100.0, |balance, tx| balance - tx.amount);
}