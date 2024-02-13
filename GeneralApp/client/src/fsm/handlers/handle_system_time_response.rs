use anyhow::Result;
use async_zmq::zmq::{self, POLLIN};
use generated::communication::*;
use protobuf::Message;

use crate::fsm::exceptions::system_time_response_exception::SystemTimeResponseError;

pub async fn handle_system_time_response(
    socket: &zmq::Socket,
) -> Result<(), SystemTimeResponseError> {
    if socket.poll(POLLIN, 10) != Ok(0) {
        let Ok(resp) = socket.recv_msg(0) else {
            return Err(SystemTimeResponseError {});
        };

        match Envelope::parse_from_bytes(&resp) {
            Ok(msg) => match msg.msgtype {
                Some(envelope::Msgtype::SystemTimeResp(_)) => {
                    log::debug!("Received SystemTimeResponse from the server {{{msg}}}");
                }
                _ => {
                    log::info!("Received unexpected response: {:?}", msg);
                }
            },
            Err(e) => {
                log::info!("Unable to deserialize response: {:?}", e);
                return Err(SystemTimeResponseError);
            }
        }
    }

    Ok(())
}
