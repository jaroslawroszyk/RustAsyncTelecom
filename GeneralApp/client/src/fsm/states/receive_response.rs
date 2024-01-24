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
                    println!("Received HeartbeatResp from the server {{{msg}}}");
                }
                Some(operation_message::Msgtype::AddUserResp(_)) => {
                    println!("Received AddUserResp from the server {{{msg}}}");
                }
                Some(operation_message::Msgtype::FooResp(_)) => {
                    println!("Received FooResp from the server {{{msg}}}");
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
}
