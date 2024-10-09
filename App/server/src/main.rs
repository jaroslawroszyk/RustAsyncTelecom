use anyhow::Result;
use server::server::Server;

async fn run_server() -> Result<()> {
    let server = Server::new().await?;

    server.run().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    log::init()?;
    run_server().await?;
    Ok(())
}
