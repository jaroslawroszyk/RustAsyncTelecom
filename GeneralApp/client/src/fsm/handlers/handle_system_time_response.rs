use anyhow::Result;
use async_zmq::zmq::{self, POLLIN};
use generated::communication::*;
use protobuf::Message;

pub async fn handle_system_time_response(socket: &zmq::Socket) -> Result<()> {
    if socket.poll(POLLIN, 10)? != 0 {
        let resp = socket.recv_msg(0)?;

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
            }
        }
    }

    Ok(())
}
