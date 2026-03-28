use async_zmq::zmq;
use zmq_handler_macro::zmq_response_handler;

use crate::fsm::exceptions::ResponseError;

/// Handles the response for the Add User request sent to the server.
/// # Errors
/// Returns `ResponseError::AddUserRespException` on receive or deserialization failure.
#[zmq_response_handler(variant = AddUserResp, error = AddUserRespException, poll_timeout_ms = 5000)]
pub async fn handle_add_user_response(socket: &zmq::Socket) -> Result<(), ResponseError> {}
