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

    pub async fn run(&self) -> Result<()> {
        let socket = self.context.socket(zmq::ROUTER)?;
        assert!(socket.bind(&self.socket_address).is_ok());

        log::info!("Server is running and waiting for messages...");
        let mut redis_state_manager = self.redis_state_manager.clone();

        run_state_machine(&socket, &mut redis_state_manager).await?;
        Ok(())
    }
}

async fn is_port_available(port: &str) -> bool {
    let addres = dotenv!("ADDRESS");
    TcpListener::bind(format!("{addres}:{port}")).await.is_ok()
}
