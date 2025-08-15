use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connection established with the server.");
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);
    let mut input = String::new();
    let mut buf = [0; 1024];

    loop {
        input.clear();

        println!("Type a message (or 'quit' to exit):");
        reader.read_line(&mut input).await?;
        if input.trim() == "quit" {
            break;
        }

        // Send to server
        stream.write_all(input.as_bytes()).await?;

        // Wait for server's reply
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            println!("Server closed connection");
            break;
        }
        println!("Server replied: {}", String::from_utf8_lossy(&buf[..n]));
    }

    Ok(())
}
