use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let (socket, _) = listener.accept().await?;
    let (mut reader, mut writer) = socket.into_split();
    println!("New client connected.");

    let read_task = tokio::spawn(async move {
        let mut buf = [0; 1024];

        loop {
            match reader.read(&mut buf).await {
                Ok(0) => {
                    println!("Client disconnected");
                    break;
                }
                Ok(n) => {
                    println!("client: {}", String::from_utf8_lossy(&buf[..n]));
                }
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    break;
                }
            }
        }
    });

    let write_task = tokio::spawn(async move {
        let stdin = io::stdin();
        let mut reader_buffer = io::BufReader::new(stdin);
        let mut input = String::new();

        loop {
            input.clear();

            if let Err(e) = reader_buffer.read_line(&mut input).await {
                eprintln!("failed to read; err = {:?}", e);
                break;
            }

            if input.trim() == "quit" {
                break;
            }

            // Write the data back
            if let Err(e) = writer.write_all(input.as_bytes()).await {
                eprintln!("failed to write to socket; err = {:?}", e);
                break;
            }
        }
    });

    // Wait for both tasks to complete
    let _ = tokio::try_join!(read_task, write_task);

    Ok(())
}
