use anyhow::Result;

use async_zmq::zmq::{self, POLLIN};
use generated::communication::*;
use protobuf::Message;

use crate::fsm::exceptions::add_user_resp_exception::AddUserRespException;

pub async fn handle_add_user_response(socket: &zmq::Socket) -> Result<(), AddUserRespException> {
    if socket.poll(POLLIN, 10) != Ok(0) {
        let Ok(resp) = socket.recv_msg(0) else {
            return Err(AddUserRespException {});
        };

        match Envelope::parse_from_bytes(&resp) {
            Ok(msg) => match msg.msgtype {
                Some(envelope::Msgtype::AddUserResp(_)) => {
                    log::debug!("Received AddUserResp from the server {{{msg}}}");
                }
                _ => {
                    log::info!("Received unexpected response: {:?}", msg);
                }
            },
            Err(_) => {
                log::info!("Unable to deserialize response");
                return Err(AddUserRespException);
            }
        }
    }

    Ok(())
}
