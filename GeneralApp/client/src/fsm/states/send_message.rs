use anyhow::Result;
use async_zmq::zmq::{self};

use generated::communication::*;
use std::time::Duration;

use crate::serializers::serialize_message;

pub async fn send_message(socket: &zmq::Socket, message: &OperationMessage) -> Result<()> {
    let serialized_msg = serialize_message(message);
    let _ = socket.send(&serialized_msg, 0).map_err(|e| {
        eprintln!("Failed to send message. ERR: {:?}", e);
        // e.into()
        //error?
    });

    println!("Sent message: {{{message}}}");
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
