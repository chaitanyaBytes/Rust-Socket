use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

type Clients = Arc<Mutex<HashMap<String, tokio::net::tcp::OwnedWriteHalf>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    println!("Chat server running on 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New client connected: {}", addr);

        let clients_clone = Arc::clone(&clients);

        tokio::spawn(async move { handle_client(socket, clients_clone, addr) });
    }
}

async fn handle_client(socket: TcpStream, clients: Clients, _addr: SocketAddr) {
    let (reader, writer) = socket.into_split();
    let mut buf_reader = io::BufReader::new(reader);
    let mut line = String::new();

    // First message from client should be their username
    if buf_reader.read_line(&mut line).await.unwrap() == 0 {
        return;
    }
    let username = line.trim().to_string();
    println!("{} joined", username);

    clients.lock().await.insert(username.clone(), writer);

    // Read messages and route them
    loop {
        line.clear();

        let _bytes = match buf_reader.read_line(&mut line).await {
            Ok(0) => {
                println!("{} disconnected", username);
                clients.lock().await.remove(&username);
                break;
            }
            Ok(n) => {
                // println!("client {}: {}", addr, String::from_utf8_lossy(&buf[..n]));
                n
            }
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                break;
            }
        };

        // Expecting format: "recipient: message"
        if let Some((to, msg)) = line.trim().split_once(":") {
            if let Some(recipient_writer) = clients.lock().await.get_mut(to) {
                let _ = recipient_writer
                    .write_all(format!("{}: {}\n", username, msg.trim()).as_bytes())
                    .await;
            } else {
                println!("{} tried to message {}, but user not found", username, to);
            }
        }
    }

    // let read_task = tokio::spawn(async move {
    //     let mut buf = [0; 1024];
    //     loop {
    //         match reader.read(&mut buf).await {
    //             Ok(0) => {
    //                 println!("Client disconnected");
    //                 break;
    //             }
    //             Ok(n) => {
    //                 println!("client {}: {}", addr, String::from_utf8_lossy(&buf[..n]));
    //             }
    //             Err(e) => {
    //                 eprintln!("failed to read from socket; err = {:?}", e);
    //                 break;
    //             }
    //         }
    //     }
    // });

    // let write_task = tokio::spawn(async move {
    //     let stdin = io::stdin();
    //     let mut reader_buffer = io::BufReader::new(stdin);
    //     let mut input = String::new();
    //     loop {
    //         input.clear();
    //         if let Err(e) = reader_buffer.read_line(&mut input).await {
    //             eprintln!("failed to read; err = {:?}", e);
    //             break;
    //         }
    //         if input.trim() == "quit" {
    //             break;
    //         }
    //         if let Err(e) = writer.write_all(input.as_bytes()).await {
    //             eprintln!("failed to write to socket; err = {:?}", e);
    //             break;
    //         }
    //     }
    // });

    // let _ = tokio::try_join!(read_task, write_task);
}
