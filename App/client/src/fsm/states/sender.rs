use anyhow::Result;
use async_zmq::zmq::{self};
use generated::communication::Envelope;
use std::time::Duration;

use crate::serializers::serialize_message;

/// Sends a protobuf message to the server via the provided ZMQ socket.
/// # Arguments
/// * `socket` - A reference to the ZMQ socket through which the message will be sent.
/// * `msg` - A reference to the protobuf message to be sent.
/// # Returns
/// A Result indicating the success or failure of the send operation.
/// # Errors
/// This function will return an error if the message cannot be serialized or if the send operation fails.
pub async fn send(socket: &zmq::Socket, msg: &Envelope) -> Result<()> {
    let serialized_data = serialize_message(msg)?;

    if let Err(e) = socket.send(serialized_data, 0) {
        logger::error!("Failed to send message via ZMQ. ERR: {:?}", e);
        return Err(e.into());
    }

    logger::info!("Sent message: {:?}", msg);

    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
