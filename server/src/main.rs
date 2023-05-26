use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buffer = [0u8; 1024];

            loop {
                // Receive message from the client
                let n = match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => break, // Connection closed
                    Ok(n) => n,
                    Err(_) => break, // Error occurred
                };

                let received_message = String::from_utf8_lossy(&buffer[..n]);
                println!("-----------------------");
                println!("Client: {}", received_message);

                // Send a response back to the client
                println!("-----------------------");
                print!("Server: ");
                io::stdout().flush();
                let mut response = String::new();
                io::stdin().read_line(&mut response);

                if let Err(err) = socket.write_all(response.as_bytes()).await {
                    eprintln!("Failed to send response to client: {}", err);
                    break;
                }
            }
        });
    }
}
