use anyhow::{bail, Result};
use async_zmq::zmq::{self, POLLIN};
use generated::communication::*;
use protobuf::Message;

use crate::fsm::exceptions::heart_beat_exception::HeartBeatException;

pub async fn handle_heart_beat_response(socket: &zmq::Socket) -> Result<()> {
    //todo implement custom erros
    let mut retries: i8 = 3;

    while retries > 0 {
        if socket.poll(POLLIN, 1000)? != 0 {
            let resp = socket.recv_msg(0)?;

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

    bail!((HeartBeatException {}));
}
