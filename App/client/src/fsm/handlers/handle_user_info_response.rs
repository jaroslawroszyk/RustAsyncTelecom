use anyhow::Result;
use async_zmq::zmq::{self, POLLIN};
use generated::communication::{envelope, Envelope};
use protobuf::Message;

use crate::fsm::exceptions::ResponseError;

pub async fn handle_user_info_response(socket: &zmq::Socket) -> Result<(), ResponseError> {
    if socket.poll(POLLIN, 10) != Ok(0) {
        let Ok(resp) = socket.recv_msg(0) else {
            return Err(ResponseError::UserInfoResponseError);
        };

        match Envelope::parse_from_bytes(&resp) {
            Ok(msg) => match msg.msgtype {
                Some(envelope::Msgtype::UserInfoResponse(_)) => {
                    log::debug!("Received UserInfoResponse from the server {{{msg}}}");
                }
                _ => {
                    log::warn!("Received unexpected response: {:?}", msg);
                }
            },
            Err(e) => {
                log::info!("Unable to deserialize response: {:?}", e);
                return Err(ResponseError::UserInfoResponseError);
            }
        }
    }

    Ok(())
}
