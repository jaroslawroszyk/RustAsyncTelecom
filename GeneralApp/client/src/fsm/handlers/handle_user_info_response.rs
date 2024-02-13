use anyhow::Result;
use async_zmq::zmq::{self, POLLIN};
use generated::communication::*;
use protobuf::Message;

use crate::fsm::exceptions::user_info_response_exception::UserInfoResponseError;

pub async fn handle_user_info_response(socket: &zmq::Socket) -> Result<(), UserInfoResponseError> {
    if socket.poll(POLLIN, 10) != Ok(0) {
        let Ok(resp) = socket.recv_msg(0) else {
            return Err(UserInfoResponseError {});
        };

        match Envelope::parse_from_bytes(&resp) {
            Ok(msg) => match msg.msgtype {
                Some(envelope::Msgtype::UserInfoResponse(_)) => {
                    log::debug!("Received UserInfoResponse from the server {{{msg}}}");
                }
                _ => {
                    log::info!("Received unexpected response: {:?}", msg);
                }
            },
            Err(e) => {
                log::info!("Unable to deserialize response: {:?}", e);
                return Err(UserInfoResponseError);
            }
        }
    }

    Ok(())
}
