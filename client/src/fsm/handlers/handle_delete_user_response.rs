use async_zmq::zmq;
use zmq_handler_macro::zmq_response_handler;

use crate::fsm::exceptions::ResponseError;

/// Handles the response for the Delete User request sent to the server.
/// # Errors
/// Returns `ResponseError::DeleteUserResponseError` on receive or deserialization failure.
#[zmq_response_handler(
    variant = DeleteUserResponse,
    error = DeleteUserResponseError,
    poll_timeout_ms = 10,
)]
pub async fn handle_delete_user_response(socket: &zmq::Socket) -> Result<(), ResponseError> {}
