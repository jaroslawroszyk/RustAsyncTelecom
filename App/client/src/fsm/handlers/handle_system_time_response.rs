use crate::fsm::exceptions::ResponseError;
use anyhow::Result;
use async_zmq::zmq::{self, POLLIN};
use generated::communication::{envelope, Envelope};
use protobuf::Message;

/// Handles the response for the System Time request sent to the server.
/// It listens for a response on the provided `ZeroMQ` socket and processes it accordingly.
/// # Errors
/// This function will return an error if it fails to receive a response, deserialize it, or
pub async fn handle_system_time_response(socket: &zmq::Socket) -> Result<(), ResponseError> {
    if socket.poll(POLLIN, 10) != Ok(0) {
        let Ok(resp) = socket.recv_msg(0) else {
            return Err(ResponseError::SystemTimeResponseError);
        };

        match Envelope::parse_from_bytes(&resp) {
            Ok(msg) => match msg.msgtype {
                Some(envelope::Msgtype::SystemTimeResp(_)) => {
                    logger::debug!("Received SystemTimeResponse from the server {{{msg}}}");
                }
                _ => {
                    logger::info!("Received unexpected response: {:?}", msg);
                }
            },
            Err(e) => {
                logger::info!("Unable to deserialize response: {:?}", e);
                return Err(ResponseError::SystemTimeResponseError);
            }
        }
    }

    Ok(())
}
