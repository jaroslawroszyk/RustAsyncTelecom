use anyhow::{bail, Ok, Result};
use async_zmq::{zmq, Context};
use dotenv_codegen::dotenv;
use redis::Commands;
use tokio::net::TcpListener;

use crate::fsm::machines::run_state_machine;

#[derive(Clone)]
pub struct Server {
    context: Context,
    socket_address: String,
    // TODO: server should have redis_client? maybe it should be a part of lib?
    /*
        We want update that counters on send from client and when it arrives?
    */
}

impl Server {
    pub async fn new() -> Result<Self> {
        if !is_port_available(dotenv!("PORT")).await {
            bail!(async_zmq::Error::EADDRINUSE);
        }

        let redis_client = redis::Client::open("redis://127.0.0.1/")?;
        let mut redis_connection = redis_client.get_connection()?;
        let _: () = redis_connection.set("user_counter", 0)?;

        Ok(Server {
            context: Context::new(),
            socket_address: (&dotenv!("IP_ADDRESS")).to_string(),
        })
    }

    pub async fn run(&self) -> Result<()> {
        let socket = self.context.socket(zmq::ROUTER)?;
        assert!(socket.bind(&self.socket_address).is_ok());

        log::info!("Server is running and waiting for messages...");
        let redis_client = redis::Client::open("redis://127.0.0.1/")?;
        let mut redis_connection = redis_client.get_connection()?;

        run_state_machine(&socket, &mut redis_connection).await?;
        Ok(())
    }
}

async fn is_port_available(port: &str) -> bool {
    let addres = dotenv!("ADDRESS");
    TcpListener::bind(format!("{addres}:{port}")).await.is_ok()
}
