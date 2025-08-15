use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connection established with the server.");

    let (mut reader, mut writer) = stream.into_split();

    // Task for reading messages from server
    let read_task = tokio::spawn(async move {
        let mut buf = [0; 1024];

        loop {
            match reader.read(&mut buf).await {
                Ok(0) => {
                    println!("Client disconnected");
                    break;
                }
                Ok(n) => {
                    println!("server: {}", String::from_utf8_lossy(&buf[..n]));
                }
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    break;
                }
            }
        }
    });

    // Task for sending messages to server
    let write_task = tokio::spawn(async move {
        let mut input = String::new();
        let stdin = io::stdin();
        let mut reader_buffer = io::BufReader::new(stdin);

        loop {
            input.clear();

            // println!("Type a message (or 'quit' to exit):");

            if let Err(e) = reader_buffer.read_line(&mut input).await {
                eprintln!("failed to read; err = {:?}", e);
                break;
            }

            if input.trim() == "quit" {
                break;
            }

            if writer.write_all(input.as_bytes()).await.is_err() {
                break; // Connection closed
            }
        }
    });

    // Wait for both tasks to complete
    let _ = tokio::try_join!(read_task, write_task);

    Ok(())
}
