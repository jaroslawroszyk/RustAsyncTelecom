use anyhow::Result;
use async_zmq::zmq;
use generated::communication::Envelope;

/// Sends a protobuf message to the client via the provided ZMQ ROUTER socket.
/// # Errors
/// Returns an error if serialization or the ZMQ send fails.
pub async fn send(socket: &zmq::Socket, msg: Envelope, identity: &[u8]) -> Result<()> {
    zmq_sender::send_router(socket, &msg, identity)?;
    Ok(())
}
