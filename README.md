# Implementation and Efficiency Analysis of Communication Protocols in Distributed Systems Using Redis, ZeroMQ, and Protobuf

## Overview

This project is a server-client application implemented in Rust as part of my bachelor's thesis. The application utilizes the `tokio` asynchronous runtime and `async_zmq` for messaging between the server and client. The goal of this project is to demonstrate the capabilities of Rust in building efficient, concurrent applications and to analyze the effectiveness of communication protocols in distributed systems.

## Technologies Used

- **Rust**: The programming language used for development.
- **Tokio**: An asynchronous runtime for Rust, enabling the implementation of concurrent I/O operations.
- **Async ZMQ**: A library for asynchronous ZeroMQ messaging.
- **Redis**: Used for data storage and message brokering.
- **Protobuf**: For efficient serialization of structured data.

## Getting Started

To get started with this project, follow these steps:

1. **Clone the repository**:

   ```bash
   git clone <repository-url>
   cd <project-directory>
   ```

2. **Build the project**:

   ```bash
   cargo build
   ```

3. **Run the server**:

   ```bash
   cargo run --bin server
   ```

4. **Run the client**:

   ```bash
   cargo run --bin client
   ```