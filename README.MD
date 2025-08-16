# Centralized Chat Server

This project is a simple centralized chat server implemented in Rust. It allows clients to connect to the server and send messages to each other through the server.

## Features

- **Client-Server Architecture**: Clients connect to a central server that routes messages to the correct recipient.
- **Asynchronous Networking**: Utilizes Rust's `tokio` library for handling multiple client connections concurrently.
- **Message Routing**: Messages are routed based on client identifiers.

## Getting Started

### Prerequisites

- Rust and Cargo installed on your system. You can install them from [rustup.rs](https://rustup.rs/).

### Running the Server

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd simple-socket
   ```

2. Run the server:
   ```bash
   cargo run --bin server
   ```

   The server will start and listen on `127.0.0.1:8080`.

### Running the Client

1. Open a new terminal and navigate to the project directory.

2. Run the client:
   ```bash
   cargo run --bin client
   ```

   Follow the prompts to enter your identifier and send messages.

## Project Structure

- `src/bin/server.rs`: Contains the server implementation.
- `src/bin/client.rs`: Contains the client implementation.
- `Cargo.toml`: Project configuration file.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.