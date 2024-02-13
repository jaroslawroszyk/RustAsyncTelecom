use anyhow::Result;
use async_zmq::zmq::{self, POLLIN};
use generated::communication::*;
use protobuf::Message;

use crate::fsm::exceptions::delete_user_response_exception::DeleteUserResponseError;

pub async fn handle_delete_user_response(
    socket: &zmq::Socket,
) -> Result<(), DeleteUserResponseError> {
    if socket.poll(POLLIN, 10) != Ok(0) {
        let Ok(resp) = socket.recv_msg(0) else {
            return Err(DeleteUserResponseError {});
        };

        match Envelope::parse_from_bytes(&resp) {
            Ok(msg) => match msg.msgtype {
                Some(envelope::Msgtype::DeleteUserResponse(resp)) => {
                    log::debug!("Received DeleteUserResponse from the server {{{resp}}}");
                    log::info!("Full: DeleteUserResponse {:?}", resp.result);
                }
                _ => {
                    log::warn!("Received unexpected response: {:?}", msg);
                }
            },
            Err(e) => {
                log::warn!("Unable to deserialize response: {:?}", e);
                return Err(DeleteUserResponseError);
            }
        }
    }

    Ok(())
}
