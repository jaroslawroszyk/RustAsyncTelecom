use anyhow::Result;
use async_zmq::zmq;

use crate::{builder::build_system_time_response, fsm::machines::send};

pub async fn state_system_time_req(socket: &zmq::Socket, identity: &[u8]) -> Result<()> {
    log::debug!("Received message: SystemTimeRequest");

    _ = send(
        &socket,
        build_system_time_response(generated::communication::Result::OK),
        &identity,
    );
    Ok(())
}
