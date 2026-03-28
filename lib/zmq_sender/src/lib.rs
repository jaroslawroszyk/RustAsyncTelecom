use anyhow::Result;
use async_zmq::zmq;
use generated::{communication::Envelope, ProtoSerialize};

/// Sends a protobuf `Envelope` over a DEALER socket (client → server).
/// # Errors
/// Returns an error if serialization or the ZMQ send fails.
pub fn send_dealer(socket: &zmq::Socket, msg: &Envelope) -> Result<()> {
    let serialized = msg.serialize()?;
    socket.send(serialized, 0)?;
    logger::info!("Sent message: {}", msg);
    Ok(())
}

/// Sends a protobuf `Envelope` over a ROUTER socket (server → client),
/// prefixing the message with the client identity frame.
/// # Errors
/// Returns an error if serialization or the ZMQ send fails.
pub fn send_router(socket: &zmq::Socket, msg: &Envelope, identity: &[u8]) -> Result<()> {
    let serialized = msg.serialize()?;
    socket
        .send(identity, zmq::SNDMORE)
        .and_then(|()| socket.send(serialized, 0))?;
    logger::info!("Sent message to client: {}", msg);
    Ok(())
}
