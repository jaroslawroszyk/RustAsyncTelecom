use anyhow::{bail, Ok, Result};
use async_zmq::{zmq, Context};
use dotenv_codegen::dotenv;
use tokio::net::TcpListener;

use crate::fsm::machines::run_state_machine;

#[derive(Clone)]
pub struct Server {
    context: Context,
    socket_address: String,
}

impl Server {
    pub async fn new() -> Result<Self> {
        if !is_port_available(dotenv!("PORT")).await {
            bail!(async_zmq::Error::EADDRINUSE);
        }

        Ok(Server {
            context: Context::new(),
            socket_address: (&dotenv!("IP_ADDRESS")).to_string(),
        })
    }

    pub async fn run(&self) -> Result<()> {
        let socket = self.context.socket(zmq::ROUTER)?;
        socket.bind(&self.socket_address)?;

        println!("Server is running and waiting for messages...");
        run_state_machine(&socket).await?;
        Ok(())
    }
}

async fn is_port_available(port: &str) -> bool {
    let addres = dotenv!("ADDRESS");
    TcpListener::bind(format!("{addres}:{port}")).await.is_ok()
}
