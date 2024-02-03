use anyhow::Result;
use async_zmq::{zmq, Context};
use client::fsm::machines::run_state_machine;

pub async fn run_client() -> Result<()> {
    let context = Context::new();
    let socket = context.socket(zmq::DEALER)?;

    run_state_machine(&socket).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    log::init()?;
    run_client().await?;
    Ok(())
}
/*
TODO: jezeli nie przyjdzie resposne na heartbeat to zamknij gniazdo! (TIMEOUT?)
*/
