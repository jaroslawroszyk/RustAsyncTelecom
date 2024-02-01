use anyhow::Result;
use server::server::Server;

async fn run_server() -> Result<()> {
    let server = Server::new().await?;
    log::debug!("Server is running and waiting for messages...");
    log::info!("Server is running and waiting for messages...");

    server.run().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run_server().await?;
    Ok(())
}
