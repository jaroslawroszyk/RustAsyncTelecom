use anyhow::Result;
use async_zmq::zmq::{self, POLLIN};
use generated::communication::*;
use protobuf::Message;

use crate::fsm::exceptions::exceptions::ResponseError;

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
                        log::debug!("Received HeartbeatResp from the server {{{msg}}}");
                        return Ok(());
                    }
                    _ => {
                        log::info!("Received unexpected response: {:?}", msg);
                    }
                },
                Err(e) => {
                    log::warn!("Unable to deserialize response: {:?}", e);
                }
            }
        }

        retries -= 1;
        log::info!("Number of retries left: {}", retries);
    }

    Ok(())
}
