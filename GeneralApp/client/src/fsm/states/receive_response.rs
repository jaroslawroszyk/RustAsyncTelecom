use anyhow::Result;
use async_zmq::zmq::{self, POLLIN};

use generated::communication::*;
use protobuf::Message;

pub async fn receive_response(socket: &zmq::Socket) -> Result<()> {
    if socket.poll(POLLIN, 10)? != 0 {
        let resp = socket.recv_msg(0)?;

        match OperationMessage::parse_from_bytes(&resp) {
            Ok(msg) => match msg.msgtype {
                Some(operation_message::Msgtype::HeartbeatResp(_)) => {
                    log::debug!("Received HeartbeatResp from the server {{{msg}}}");
                }
                Some(operation_message::Msgtype::AddUserResp(_)) => {
                    log::debug!("Received AddUserResp from the server {{{msg}}}");
                }
                Some(operation_message::Msgtype::FooResp(_)) => {
                    log::debug!("Received FooResp from the server {{{msg}}}");
                }
                _ => {
                    log::debug!("Received unexpected response: {:?}", msg);
                }
            },
            Err(e) => {
                log::debug!("Unable to deserialize response: {:?}", e);
            }
        }
    }

    Ok(())
}
