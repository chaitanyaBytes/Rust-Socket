# Simple Socket Project

## Overview

The Simple Socket project is a basic implementation of a TCP client-server model using Rust and the Tokio asynchronous runtime. It demonstrates how to establish a connection between a client and a server, send and receive messages, and handle multiple connections concurrently.

## Features

- Asynchronous communication using Tokio
- Concurrent handling of multiple client connections
- Simple message exchange between client and server

## Prerequisites

- Rust and Cargo installed on your system. You can download them from [rust-lang.org](https://www.rust-lang.org/).

## Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd simple-socket
   ```

2. Build the project using Cargo:
   ```bash
   cargo build
   ```

## Running the Server

To start the server, run the following command:

```bash
cargo run --bin server
```

The server will start listening on `127.0.0.1:8080` for incoming client connections.

## Running the Client

To start the client, run the following command:

```bash
cargo run --bin client
```

The client will attempt to connect to the server at `127.0.0.1:8080`. Once connected, you can type messages to send to the server. Type `quit` to disconnect.

## Dependencies

The project uses the following dependencies:

- `tokio`: For asynchronous runtime and networking

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
