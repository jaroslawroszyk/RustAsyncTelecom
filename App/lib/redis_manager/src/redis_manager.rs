#![allow(clippy::missing_errors_doc)]

use anyhow::Result;
use namespace::APP_STATE_NS;
use redis::Commands;

pub mod namespace;

#[derive(Clone)]
pub struct RedisStateManager {
    client: redis::Client,
}

impl RedisStateManager {
    #[allow(clippy::unused_async)]
    pub async fn new() -> Result<Self> {
        let redis_client = redis::Client::open("redis://127.0.0.1/")?;
        Ok(Self {
            client: redis_client,
        })
    }

    #[allow(clippy::unused_async)]
    pub async fn set(&mut self, namespace: &str, key: &str, value: &str) -> Result<()> {
        let mut con = self.client.get_connection()?;
        let _: () = con.hset(namespace, key, value)?;
        Ok(())
    }

    #[allow(clippy::unused_async)]
    pub async fn get(&mut self, namespace: &str, key: &str) -> Result<String> {
        let mut con: redis::Connection = self.client.get_connection()?;
        let result = con.hget(namespace, key)?;
        Ok(result)
    }

    #[allow(clippy::unused_async)]
    pub async fn get_all_from_ns(&mut self, namespace: &str) -> Result<Vec<String>> {
        let mut con: redis::Connection = self.client.get_connection()?;
        let result = con.hgetall(namespace)?;
        Ok(result)
    }

    #[allow(clippy::unused_async)]
    pub async fn delete(&mut self, namespace: &str, key: &str) -> Result<()> {
        let mut con = self.client.get_connection()?;
        let _: () = con.hdel(namespace, key)?;
        Ok(())
    }

    pub async fn reset_counter(&mut self, namespace: &str) -> Result<()> {
        self.set_counter(namespace, 0).await
    }

    pub async fn restore_state(&mut self) -> Result<()> {
        let mut con = self.client.get_connection()?;
        let counter: i32 = con.get(APP_STATE_NS)?;
        self.set_counter("*", counter).await?;
        Ok(())
    }

    pub async fn save_state(&mut self, counter: i32) -> Result<()> {
        self.set_counter("*", counter).await
    }

    #[allow(clippy::unused_async)]
    pub async fn increment_counter(&mut self, key: &str) -> Result<()> {
        let mut con = self.client.get_connection()?;
        let _: () = con.incr(key, 1)?;
        Ok(())
    }

    #[allow(clippy::unused_async)]
    pub async fn set_counter(&mut self, namespace: &str, data: i32) -> Result<()> {
        let mut con = self.client.get_connection()?;
        let _: () = con.set(namespace, data)?;
        Ok(())
    }

    #[allow(clippy::unused_async)]
    pub async fn get_counter(&mut self, namespace: &str) -> Result<i32> {
        let mut con: redis::Connection = self.client.get_connection()?;
        let counter: i32 = con.get(namespace)?;
        Ok(counter)
    }

    #[allow(clippy::unused_async)]
    pub async fn reset_all_counters(&mut self) -> Result<()> {
        let mut con = self.client.get_connection()?;
        let keys: Vec<String> = con.keys("*")?;

        for key in keys {
            let _: () = con.set(&key, 0)?;
        }

        Ok(())
    }
}
