mod data;
mod utility;
mod models;
mod network;

use crate::network::{
    peerDiscovery::discover_peer,
    peerCommServer::start_server,
    peerCommClient::send_message
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Start the server (runs forever in background)
    tokio::spawn(async {
        if let Err(e) = start_server().await {
            eprintln!("Server error: {}", e);
        }
    });

    // Wait a second for the server to be ready
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // Discover a peer via UDP
    if let Some(peer_addr) = discover_peer().await? {
        send_message(peer_addr).await?;
    } else {
        println!("No peers discovered.");
    }

    Ok(())
}
