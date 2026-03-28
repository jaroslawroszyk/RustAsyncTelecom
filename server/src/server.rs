use anyhow::{bail, Ok, Result};
use async_zmq::{zmq, Context};
use dotenv_codegen::dotenv;
use redis_manager::RedisStateManager;
use tokio::net::TcpListener;

use crate::fsm::machines::run_state_machine;

#[derive(Clone)]
pub struct Server {
    context: Context,
    socket_address: String,
    redis_state_manager: RedisStateManager,
}

impl Server {
    /// Creates a new instance of the `Server` struct, initializing the `ZeroMQ` context and setting up the socket address.
    /// It also initializes the `RedisStateManager` to manage the server's state in Redis. The function checks if the specified port is available before proceeding.
    /// # Errors
    /// This function will return an error if the specified port is already in use, or if there are issues initializing the `RedisStateManager` or any other operations that may fail during the setup
    pub async fn new() -> Result<Self> {
        if !is_port_available(dotenv!("PORT")).await {
            bail!(async_zmq::Error::EADDRINUSE);
        }

        let mut redis_state_manager = RedisStateManager::new().await?;
        redis_state_manager
            .set_counter("app_state_counter", 0)
            .await?;

        Ok(Server {
            context: Context::new(),
            socket_address: (&dotenv!("IP_ADDRESS")).to_string(),
            redis_state_manager,
        })
    }

    /// Starts the server, binds to the specified socket address, and runs the state machine to handle incoming messages.
    /// The server will listen for messages from clients and process them according to the defined state machine.
    /// # Errors
    /// This function will return an error if it fails to bind to the socket address, or if any of the operations within the state machine fail, such as receiving messages, processing them, or sending responses.
    pub async fn run(&self) -> Result<()> {
        let socket = self.context.socket(zmq::ROUTER)?;
        socket.bind(&self.socket_address)?;

        logger::info!("Server is running and waiting for messages...");
        let mut redis_state_manager = self.redis_state_manager.clone();

        run_state_machine(&socket, &mut redis_state_manager).await?;
        Ok(())
    }
}

async fn is_port_available(port: &str) -> bool {
    let addres = dotenv!("ADDRESS");
    TcpListener::bind(format!("{addres}:{port}")).await.is_ok()
}
