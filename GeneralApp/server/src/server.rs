use anyhow::{bail, Ok, Result};
use async_zmq::{zmq, Context};
use dotenv_codegen::dotenv;
use redis::Commands;
use tokio::net::TcpListener;

use crate::fsm::machines::run_state_machine;

//TODO: stworzyc plik z namespacami w libce redisa
const APP_STATE_NS: &str = "app_state_counter";

//TODO: extract it to lib?
#[derive(Clone)]
pub struct RedisStateManager {
    client: redis::Client,
}

impl RedisStateManager {
    pub async fn new() -> Result<Self> {
        let redis_client = redis::Client::open("redis://127.0.0.1/")?;
        Ok(Self {
            client: redis_client,
        })
    }

    pub async fn set_counter(&mut self, namespace: &str, data: i32) -> Result<()> {
        let mut con = self.client.get_connection()?;
        con.set(namespace, data)?;
        Ok(())
    }

    pub async fn reset_counter(&mut self, namespace: &str) -> Result<()> {
        self.set_counter(namespace, 0).await
    }

    pub async fn restore_state(&mut self) -> Result<()> {
        let mut con = self.client.get_connection()?;
        let counter: i32 = con.get(APP_STATE_NS)?;
        self.set_counter("*", counter).await?; // to zadziala?
        Ok(())
    }

    pub async fn save_state(&mut self, counter: i32) -> Result<()> {
        self.set_counter("*", counter).await
    }

    pub async fn increment_counter(&mut self, key: &str) -> Result<()> {
        let mut con = self.client.get_connection()?;
        let _: () = con.incr(key, 1)?;
        Ok(())
    }

    pub async fn get_counter(&mut self, key: &str) -> Result<i32> {
        let mut con = self.client.get_connection()?;
        let counter: i32 = con.get(key)?;
        Ok(counter)
    }

    pub async fn reset_all_counters(&mut self) -> Result<()> {
        let mut con = self.client.get_connection()?;
        let keys: Vec<String> = con.keys("*")?;

        for key in keys {
            con.set(&key, 0)?;
        }

        Ok(())
    }
}

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
        // redis_state_manager.reset_all_counters().await?;
        redis_state_manager
            .set_counter("app_state_counter", 0)
            .await?; // do trzymania stanu aplikacji inkrementuj ten app state counter w kazdym msg?
                     // redis_state_manager.restore_state().await?;
                     // TODO: Czy tutaj powinien byc restore?

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
