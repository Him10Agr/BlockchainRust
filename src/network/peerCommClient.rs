use tokio::{net::TcpStream, io::AsyncWriteExt};
use std::net::SocketAddr;

pub async fn send_message(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut socket = TcpStream::connect(addr).await?;
    let msg = "Hello from client!";
    socket.write_all(msg.as_bytes()).await?;
    println!("Sent TCP message to {}", addr);
    Ok(())
}
