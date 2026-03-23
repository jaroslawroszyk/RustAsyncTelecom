use anyhow::Result;

use async_zmq::zmq::{self, POLLIN};
use generated::communication::*;
use protobuf::Message;

use crate::fsm::exceptions::ResponseError;

pub async fn handle_add_user_response(socket: &zmq::Socket) -> Result<(), ResponseError> {
    if socket.poll(POLLIN, 10) != Ok(0) {
        let Ok(resp) = socket.recv_msg(0) else {
            return Err(ResponseError::AddUserRespException);
        };

        match Envelope::parse_from_bytes(&resp) {
            Ok(msg) => match msg.msgtype {
                Some(envelope::Msgtype::AddUserResp(_)) => {
                    logger::debug!("Received AddUserResp from the server {{{msg}}}");
                }
                _ => {
                    logger::info!("Received unexpected response: {:?}", msg);
                }
            },
            Err(_) => {
                logger::info!("Unable to deserialize response");
                return Err(ResponseError::AddUserRespException);
            }
        }
    }

    Ok(())
}
