use anyhow::Result;
use async_zmq::zmq;
use generated::communication::Envelope;
use zmq::SNDMORE;

use crate::serializers::serialize_message;

/// Sends a protobuf message to the client via the provided ZMQ socket.
/// # Errors
/// This function will return an error if the message cannot be serialized or if the send operation fails
pub async fn send(socket: &zmq::Socket, msg: Envelope, identity: &[u8]) -> Result<()> {
    let serialized_msg = serialize_message(&msg)?;

    let result = socket
        .send(identity, SNDMORE)
        .and_then(|()| socket.send(serialized_msg, 0));

    match result {
        Ok(()) => logger::info!("Send message to the client: {}", msg),
        Err(_) => logger::error!("Error sending msg"),
    }
    Ok(())
}
