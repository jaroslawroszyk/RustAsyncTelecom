use anyhow::Result;
use async_zmq::zmq;
use generated::communication::AddUserReq;
use redis_manager::{namespace::USERS_NS, RedisStateManager};

use crate::{builder::build_add_user_response, fsm::machines::send};

/// Handles the `AddUserReq` message received from the client.
/// It adds a new user to the Redis database based on the user ID and user name provided
/// in the request and sends back a response indicating success or failure.
/// # Errors
/// This function will return an error if it fails to add the user to the Redis database or
/// if it fails to send the response back to the client.
pub async fn state_add_user_req(
    socket: &zmq::Socket,
    msg: &AddUserReq,
    redis_state_manager: &mut RedisStateManager,
    identity: &[u8],
) -> Result<()> {
    logger::debug!("Received message: AddUserRequest {{{msg}}}");

    _ = send(
        socket,
        build_add_user_response(msg, generated::communication::Result::OK),
        identity,
    );

    redis_state_manager
        .set(USERS_NS, &msg.user_id.to_string(), &msg.user_name)
        .await?;

    logger::debug!(
        "Value for userId: {} is: {}",
        msg.user_id,
        redis_state_manager
            .get(USERS_NS, &msg.user_id.to_string())
            .await?
    );
    Ok(())
}
