use anyhow::Result;
use async_zmq::zmq;

use crate::{builder::build_system_time_response, fsm::machines::send};

/// Handles the `SystemTimeRequest` message received from the client.
/// It retrieves the current system time and sends back a response containing the system time or an error
/// # Errors
/// This function will return an error if it fails to retrieve the system time or if it fails to send the response back to the client.
pub async fn state_system_time_req(socket: &zmq::Socket, identity: &[u8]) -> Result<()> {
    logger::debug!("Received message: SystemTimeRequest");

    _ = send(
        socket,
        build_system_time_response(generated::communication::Result::OK),
        identity,
    );
    Ok(())
}
