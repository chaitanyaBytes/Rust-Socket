use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut username = String::new();
    println!("Enter your username:");
    let stdin = io::stdin();
    let mut reader_buffer = io::BufReader::new(stdin);
    reader_buffer.read_line(&mut username).await?;
    username = username.trim().to_string();

    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (reader, mut writer) = stream.into_split();

    // Send username first
    writer
        .write_all(format!("{}\n", username).as_bytes())
        .await?;

    // Spawn task to read incoming messages
    tokio::spawn(async move {
        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new();
        while buf_reader.read_line(&mut line).await.unwrap() > 0 {
            print!("Incoming: {}", line);
            line.clear();
        }
    });

    // Main loop to send messages
    let mut stdin_reader = BufReader::new(io::stdin());
    let mut input = String::new();
    loop {
        input.clear();
        stdin_reader.read_line(&mut input).await?;
        // Format: recipient: message
        writer.write_all(input.as_bytes()).await?;
    }
}
