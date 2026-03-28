use async_zmq::zmq;
use zmq_handler_macro::zmq_response_handler;

use crate::fsm::exceptions::ResponseError;

/// Handles the response for the User Info request sent to the server.
/// # Errors
/// Returns `ResponseError::UserInfoResponseError` on receive or deserialization failure.
#[zmq_response_handler(
    variant = UserInfoResponse,
    error = UserInfoResponseError,
    poll_timeout_ms = 10,
)]
pub async fn handle_user_info_response(socket: &zmq::Socket) -> Result<(), ResponseError> {}
