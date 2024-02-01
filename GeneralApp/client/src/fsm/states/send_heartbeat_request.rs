use anyhow::Result;
use async_zmq::zmq::{self};

use std::time::Duration;

use crate::{msg_builder::build_heartbeat_req_message, serializers::serialize_message};

pub async fn send_heartbeat_request(socket: &zmq::Socket) -> Result<()> {
    let heartbeat_msg = build_heartbeat_req_message();
    let serialized_heartbeat_msg = serialize_message(&heartbeat_msg);

    if let Err(e) = socket.send(&serialized_heartbeat_msg, 0) {
        log::info!("Failed to send HeartbeatReq message. ERR: {:?}", e);
        return Err(e.into());
    }

    log::info!("Sent HeartbeatReq message");
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
