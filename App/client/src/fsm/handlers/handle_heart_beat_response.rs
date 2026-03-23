use anyhow::Result;
use async_zmq::zmq::{self, POLLIN};
use generated::communication::{envelope, Envelope};
use protobuf::Message;

use crate::fsm::exceptions::ResponseError;

/// Handles the response for the Heartbeat request sent to the server.
/// It listens for a response on the provided `ZeroMQ` socket and processes it accordingly.
/// The function will retry up to 3 times if it does not receive a response within the specified timeout.
/// # Errors
/// This function will return an error if it fails to receive a response, deserialize it, or if the response is not of the expected type after exhausting all retries.
pub async fn handle_heart_beat_response(socket: &zmq::Socket) -> Result<(), ResponseError> {
    let mut retries: i8 = 3;

    while retries > 0 {
        if socket.poll(POLLIN, 1000) != Ok(0) {
            let Ok(resp) = socket.recv_msg(0) else {
                return Err(ResponseError::HeartBeatException);
            };

            match Envelope::parse_from_bytes(&resp) {
                Ok(msg) => match msg.msgtype {
                    Some(envelope::Msgtype::HeartbeatResp(_)) => {
                        logger::debug!("Received HeartbeatResp from the server {{{msg}}}");
                        return Ok(());
                    }
                    _ => {
                        logger::info!("Received unexpected response: {:?}", msg);
                    }
                },
                Err(e) => {
                    logger::warn!("Unable to deserialize response: {:?}", e);
                }
            }
        }

        retries -= 1;
        logger::info!("Number of retries left: {}", retries);
    }

    Ok(())
}
