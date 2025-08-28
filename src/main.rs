mod data;
mod utility;
mod models;
mod network;

use web3::contract::{Contract, Options};
use web3::types::{Address, BlockId, BlockNumber};
use web3::Transport;
use web3::transports::Http;

use hex_literal::hex;

async fn interact_with_contact<T: Transport>(transport: T) {

    let web3 = web3::Web3::new(transport);
    let contract_addr = Address::from_slice(&hex!("89336be900ca9f5a9913be4577276249e10faf20"));
    //let contract_addr = "0x89336be900ca9f5a9913be4577276249e10faf206f8f9a106af4ef4195af9f2d";
    let contract_abi = include_bytes!("../target/ink/counter/counter.json");

    let contract = Contract::from_json(web3.eth(), contract_addr, contract_abi).unwrap();

    let value: i32 = contract.query("get_value", (), None, Options::default(), None).await.unwrap();
    println!("Value: {}", value);

    let transaction = contract.call("increment", 5, Address::from_slice("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266".as_bytes()), Options::default()).await.unwrap();
    println!("Transaction: {:?}", transaction);

    let value: i32 = contract.query("get_value", (), None, Options::default(), None).await.unwrap();
    println!("Value: {}", value);

    let transaction_receipt = web3.eth().transaction_receipt(transaction).await.unwrap().expect("Unable to read transaction hash");
    println!("Transaction receipt: {:#?}", transaction_receipt);

    let block_number = BlockNumber::Number(web3.eth().block_number().await.unwrap());
    println!("Block Number: {:#?}", block_number);
    
    let block_id = BlockId::Number(block_number);
    println!("Block Id: {:#?}", block_id);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    let http = Http::new("http://localhost:8545").expect("Failed to create HTTP connection");
    interact_with_contact(http).await;
    return Ok(());
}
