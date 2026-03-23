use anyhow::Result;
use async_zmq::zmq::{self};
use generated::communication::Envelope;

use std::time::Duration;

use crate::serializers::serialize_message;

pub async fn send(socket: &zmq::Socket, msg: &Envelope) -> Result<()> {
    let serialize_msg = serialize_message(msg);

    if let Err(e) = socket.send(&serialize_msg, 0) {
        log::info!("Failed to send {} message. ERR: {:?}", msg, e);
        return Err(e.into());
    }

    log::info!("Sent {} message", msg);
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
