use anyhow::Result;
use async_zmq::zmq::{self};

use std::time::Duration;

use crate::{msg_builder::build_system_time_req, serializers::serialize_message};

pub async fn send_system_time_req(socket: &zmq::Socket) -> Result<()> {
    let system_time_req: generated::communication::Envelope = build_system_time_req();
    let serialized_foo_req = serialize_message(&system_time_req);

    if let Err(e) = socket.send(&serialized_foo_req, 0) {
        log::info!("Failed to send SystemTimeReq message. ERR: {:?}", e);
        return Err(e.into());
    }

    log::info!("Sent SystemTimeReq message");
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
