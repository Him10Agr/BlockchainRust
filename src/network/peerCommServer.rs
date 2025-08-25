use tokio::net::{TcpListener, UdpSocket};
use tokio::io::{AsyncReadExt};

pub async fn start_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Spawn TCP server
    tokio::spawn(async {
        if let Err(e) = run_tcp_server().await {
            eprintln!("TCP server error: {}", e);
        }
    });

    // Spawn UDP listener for discovery
    tokio::spawn(async {
        if let Err(e) = listen_for_discovery().await {
            eprintln!("UDP discovery error: {}", e);
        }
    });

    // Keep the main server alive
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}

async fn run_tcp_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    println!("TCP server running on 127.0.0.1:8000");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Accepted TCP connection from {}", addr);

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            let n = socket.read(&mut buf).await.unwrap_or(0);
            println!("Received over TCP: {}", String::from_utf8_lossy(&buf[..n]));
        });
    }
}

async fn listen_for_discovery() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let socket = UdpSocket::bind("0.0.0.0:8000").await?;
    println!("Listening for discovery on UDP 0.0.0.0:8000");

    let mut buf = [0u8; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let msg = std::str::from_utf8(&buf[..len])?;
        if msg == "DISCOVER_PEERS" {
            println!("Received discovery ping from {}", addr);

            // Reply with TCP address (you could use a public IP here)
            let response = "127.0.0.1:8000";
            socket.send_to(response.as_bytes(), addr).await?;
        }
    }
}
