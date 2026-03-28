use anyhow::Result;
use async_zmq::zmq;
use generated::communication::Envelope;
use std::time::Duration;

/// Sends a protobuf message to the server via the provided ZMQ DEALER socket,
/// then waits 1 second before returning to pace the request flow.
/// # Errors
/// Returns an error if serialization or the ZMQ send fails.
pub async fn send(socket: &zmq::Socket, msg: &Envelope) -> Result<()> {
    zmq_sender::send_dealer(socket, msg)?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(())
}
