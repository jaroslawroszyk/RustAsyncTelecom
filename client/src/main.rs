use anyhow::Result;
use async_zmq::{zmq, Context};
use client::fsm::machines::run_state_machine;

/// This function runs the client state machine, which handles the communication with the server.
/// It creates a ZMQ context and socket, and then calls the state machine to process messages from the server.
/// Returns a Result indicating success or failure of the client operation.
/// # Errors
/// This function may return an error if there is an issue with the ZMQ socket or if the state machine encounters an error while processing messages.
pub async fn run_client() -> Result<()> {
    let context = Context::new();
    let socket = context.socket(zmq::DEALER)?;

    run_state_machine(&socket).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    logger::init()?;
    run_client().await?;
    Ok(())
}
