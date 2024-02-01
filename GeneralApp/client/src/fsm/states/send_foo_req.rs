use anyhow::Result;
use async_zmq::zmq::{self};

use std::time::Duration;

use crate::{msg_builder::build_foo_req, serializers::serialize_message};

//tmp msg
pub async fn send_foo_req(socket: &zmq::Socket) -> Result<()> {
    let foo_req_msg = build_foo_req();
    let serialized_foo_req = serialize_message(&foo_req_msg);

    if let Err(e) = socket.send(&serialized_foo_req, 0) {
        log::info!("Failed to send HeartbeatReq message. ERR: {:?}", e);
        return Err(e.into());
    }

    log::info!("Sent FooReq message");
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
