use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("New client connected.");

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let stdin = io::stdin();
            let mut reader = io::BufReader::new(stdin);
            let mut input = String::new();

            // In a loop, read data from the socket and write the data back.
            loop {
                match socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => {
                        println!("Client disconnected.");
                        return;
                    }
                    Ok(n) => {
                        println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
                    }
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                input.clear();

                if let Err(e) = reader.read_line(&mut input).await {
                    eprintln!("failed to read; err = {:?}", e);
                    return;
                }

                if input.trim() == "quit" {
                    break;
                }

                // Write the data back
                if let Err(e) = socket.write_all(input.as_bytes()).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
