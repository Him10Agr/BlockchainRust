use std::net::SocketAddr;

use tokio::net::UdpSocket;

pub async fn discover_peer() -> Result<Option<SocketAddr>, Box<dyn std::error::Error + Send + Sync>> {

    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.set_broadcast(true)?;

    socket.send_to(b"DISCOVER_PEERS", "255.255.255.255:8000").await?;

    let mut buf = [0u8; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let msg = std::str::from_utf8(&buf[..len])?;
        println!("Recieved response from {}: {}", addr, msg);
    }    
}