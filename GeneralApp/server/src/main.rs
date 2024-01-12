use anyhow::Result;
use server::run_server;

mod server;

#[tokio::main]
async fn main() -> Result<()> {
    run_server().await?;
    Ok(())
}
