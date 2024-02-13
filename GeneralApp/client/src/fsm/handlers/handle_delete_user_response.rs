use anyhow::Result;
use async_zmq::zmq::{self, POLLIN};
use generated::communication::*;
use protobuf::Message;

pub async fn handle_delete_user_response(socket: &zmq::Socket) -> Result<()> {
    if socket.poll(POLLIN, 10)? != 0 {
        let resp = socket.recv_msg(0)?;

        match Envelope::parse_from_bytes(&resp) {
            Ok(msg) => match msg.msgtype {
                Some(envelope::Msgtype::DeleteUserResponse(resp)) => {
                    log::debug!("Received DeleteUserResponse from the server {{{resp}}}");
                    log::info!("Full: DeleteUserResponse {:?}", resp.result)
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
