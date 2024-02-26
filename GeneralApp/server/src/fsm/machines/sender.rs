use anyhow::Result;
use async_zmq::zmq;
use generated::communication::*;
use zmq::SNDMORE;

use crate::serializers::serialize_message;

pub fn send(socket: &zmq::Socket, msg: Envelope, identity: &[u8]) -> Result<()> {
    let serialized_msg = serialize_message(&msg);

    let result = socket
        .send(&identity, SNDMORE)
        .and_then(|_| socket.send(serialized_msg, 0));

    match result {
        Ok(_) => log::info!("Send message to the client: {}", msg),
        Err(_) => log::error!("Error sending msg"),
    }
    Ok(())
}
