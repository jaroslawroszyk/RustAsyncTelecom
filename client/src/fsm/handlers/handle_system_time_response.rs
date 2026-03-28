use async_zmq::zmq;
use zmq_handler_macro::zmq_response_handler;

use crate::fsm::exceptions::ResponseError;

/// Handles the response for the System Time request sent to the server.
/// # Errors
/// Returns `ResponseError::SystemTimeResponseError` on receive or deserialization failure.
#[zmq_response_handler(
    variant = SystemTimeResp,
    error = SystemTimeResponseError,
    poll_timeout_ms = 5000,
)]
pub async fn handle_system_time_response(socket: &zmq::Socket) -> Result<(), ResponseError> {}
