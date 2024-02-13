use anyhow::Result;
use async_zmq::zmq::{self};
use generated::communication::Envelope;

use std::time::Duration;

use crate::serializers::serialize_message;

pub fn build_delete_user_req() -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_delete_user_request();

    req.user_id = 1;
    req.username = "Alice".to_owned();

    msg
}

pub async fn send_delete_user_request(socket: &zmq::Socket) -> Result<()> {
    let delete_user_req = build_delete_user_req();
    let serialized_delete_user_req = serialize_message(&delete_user_req);

    if let Err(e) = socket.send(&serialized_delete_user_req, 0) {
        log::info!("Failed to send DeleteUserReq message. ERR: {:?}", e);
        return Err(e.into());
    }

    log::info!("Sent DeleteUserReq message");
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
