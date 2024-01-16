use anyhow::Result;
use async_zmq::zmq::{self, POLLIN};

use generated::company::*;
use protobuf::Message;
use std::time::Duration;

use crate::{
    serializers::serialize_message,
};


pub async fn sending_add_user_req(
    socket: &zmq::Socket,
    iter: &mut std::slice::Iter<'_, SomeMsg>,
) -> Result<()> {
    if let Some(message) = iter.next() {
        let serialized_msg = serialize_message(message);

        if let Err(e) = socket.send(&serialized_msg, 0) {
            eprintln!("Failed to send message. ERR: {:?}", e);
            return Err(e.into());
        }

        println!("Sent message: {{{message}}}");
        tokio::time::sleep(Duration::from_secs(1)).await;

        if socket.poll(POLLIN, 10)? != 0 {
            let resp = socket.recv_msg(0)?;

            match SomeMsg::parse_from_bytes(&resp) {
                Ok(msg) => match msg.msgtype {
                    Some(some_msg::Msgtype::AddUserResp(_)) => {
                        println!("Received AddUserResp from the server {{{msg}}}");
                    }
                    Some(some_msg::Msgtype::HeartbeatResp(_)) => {
                        println!("Received HeartbeatResp from the server {{{msg}}}");
                    }
                    _ => {
                        eprintln!("Received unexpected response: {:?}", msg);
                    }
                },
                Err(e) => {
                    eprintln!("Unable to deserialize response: {:?}", e);
                }
            }
        }

        Ok(())
    } else {
        Err(anyhow::anyhow!("No more messages to send"))
    }
}