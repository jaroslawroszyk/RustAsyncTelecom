use anyhow::Result;

use async_zmq::zmq::{self, POLLIN};
use generated::communication::{envelope, Envelope};
use protobuf::Message;

use crate::fsm::exceptions::ResponseError;

/// Handles the response for the Add User request sent to the server.
/// It listens for a response on the provided `ZeroMQ` socket and processes it accordingly.
/// # Errors
/// This function will return an error if it fails to receive a response, deserialize it, or if the response is not of the expected type.
pub async fn handle_add_user_response(socket: &zmq::Socket) -> Result<(), ResponseError> {
    if socket.poll(POLLIN, 10) != Ok(0) {
        let Ok(resp) = socket.recv_msg(0) else {
            return Err(ResponseError::AddUserRespException);
        };

        let Ok(msg) = Envelope::parse_from_bytes(&resp) else {
            logger::info!("Unable to deserialize response");
            return Err(ResponseError::AddUserRespException);
        };

        match msg.msgtype {
            Some(envelope::Msgtype::AddUserResp(_)) => {
                logger::debug!("Received AddUserResp from the server {{{msg}}}");
            }
            _ => {
                logger::info!("Received unexpected response: {:?}", msg);
            }
        }
    }

    Ok(())
}
