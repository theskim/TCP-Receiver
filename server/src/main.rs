use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    let (mut socket, _) = listener.accept().await?;
    
    let mut buffer = [0u8; 1024];

    // Receive message from the client
    let n = match socket.read(&mut buffer).await {
        Ok(n) if n == 0 => return Ok(()), // Connection closed
        Ok(n) => n,
        Err(_) => return Ok(()), // Error occurred
    };

    let received_message = String::from_utf8_lossy(&buffer[..n]);
    println!("-----------------------");
    println!("Client: {}", received_message);

    // Server sends a response back to the client
    println!("-----------------------");
    let response = "Arsenal bottled the league";
    socket.write_all(response.as_bytes()).await?;

    Ok(())
}
