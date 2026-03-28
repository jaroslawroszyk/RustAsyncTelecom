use anyhow::Result;
use async_zmq::zmq;
use generated::communication::UserInfoRequest;
use redis_manager::{namespace::USERS_NS, RedisStateManager};

use crate::{builder::build_user_info_response, fsm::machines::send};

/// Handles the `UserInfoRequest` message received from the client.
/// It retrieves the user information from the Redis database based on the user ID provided in the request
/// and sends back a response containing the user information or an error message if the user is not found.
/// # Errors
/// This function will return an error if it fails to retrieve user information from the Redis database or if it fails to send the response back to the client.
pub async fn state_user_info_req(
    socket: &zmq::Socket,
    msg: &UserInfoRequest,
    redis_state_manager: &mut RedisStateManager,
    identity: &[u8],
) -> Result<()> {
    logger::debug!("Received message: UserInfoRequest {{{msg}}}");
    let username_from_db = redis_state_manager
        .get(USERS_NS, &msg.user_id.to_string())
        .await;

    let response = match username_from_db {
        Ok(username) => build_user_info_response(username, generated::communication::Result::OK),
        Err(_) => {
            build_user_info_response("NULL".to_string(), generated::communication::Result::ERR)
        }
    };
    send(socket, response, identity).await?;
    Ok(())
}
