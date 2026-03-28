use async_zmq::zmq;
use zmq_handler_macro::zmq_response_handler;

use crate::fsm::exceptions::ResponseError;

/// Handles the response for the Heartbeat request sent to the server.
/// Retries up to 3 times if no response is received within the poll timeout.
/// # Errors
/// Returns `ResponseError::HeartBeatException` if the socket receive fails.
#[zmq_response_handler(
    variant = HeartbeatResp,
    error = HeartBeatException,
    poll_timeout_ms = 1000,
    retries = 3,
)]
pub async fn handle_heart_beat_response(socket: &zmq::Socket) -> Result<(), ResponseError> {}
