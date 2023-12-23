use async_zmq::Result;
use server::run_server;

mod server;
mod port_error;

#[tokio::main]
async fn main() -> Result<()> {
    run_server().await?;
    Ok(())
}
