use async_zmq::Result;
use client::run_client;

mod client;

#[tokio::main]
async fn main() -> Result<()> {
    run_client().await?;
    Ok(())
}
