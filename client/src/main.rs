use std::error::Error;
use std::io::{self, Write};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "server:8000";

    // Connect to the server
    let mut stream = TcpStream::connect(addr).await?;
    println!("Connected to server!");

    // Create a buffer to store received messages
    let mut buffer = [0u8; 1024];

    // Client Message
    println!("-----------------------");
    print!("Client: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Send the input as a message to the server
    stream.write_all(input.trim().as_bytes()).await?;

    // Check if the user wants to exit the chat
    if input.trim().eq_ignore_ascii_case("exit") {
        return Ok(());
    }

    // Receive message from the server
    let n = match stream.read(&mut buffer).await {
        Ok(n) if n == 0 => {
            println!("Server closed the connection.");
            return Ok(());
        }
        Ok(n) => n,
        Err(err) => {
            eprintln!("Failed to receive message from server: {}", err);
            return Ok(());
        }
    };

    let received_message = String::from_utf8_lossy(&buffer[..n]);
    println!("-----------------------");
    println!("Server: {}", received_message);
    println!("-----------------------");
    Ok(())
}