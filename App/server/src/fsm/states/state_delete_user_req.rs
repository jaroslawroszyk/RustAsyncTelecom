use anyhow::Result;
use async_zmq::zmq;
use generated::communication::DeleteUserRequest;
use redis_manager::{namespace::USERS_NS, RedisStateManager};

use crate::{builder::build_delete_user_response, fsm::machines::send};

/// Deletes a user from the state manager and sends a response.
///
/// # Errors
/// Returns an error if Redis operations fail or if the ZMQ send fails.
pub async fn state_delete_user_req(
    socket: &zmq::Socket,
    msg: &DeleteUserRequest,
    redis_state_manager: &mut RedisStateManager,
    identity: &[u8],
) -> Result<()> {
    logger::debug!(
        "Received message: DeleteUserRequest {{user_id: {}}}",
        msg.user_id
    );

    let delete_user_id = msg.user_id.to_string();
    let get_from_redis = redis_state_manager.get(USERS_NS, &delete_user_id).await;
    let result_redis_call = redis_state_manager.delete(USERS_NS, &delete_user_id).await;

    let response = if let Ok(()) = result_redis_call {
        let user_name = get_from_redis.unwrap_or_else(|_| "Unknown".to_string());

        logger::debug!("Deleted user: {delete_user_id} name: {user_name} from DB");
        build_delete_user_response(msg, &user_name, generated::communication::Result::OK)
    } else {
        logger::error!("Failed to delete user {delete_user_id} from Redis");
        build_delete_user_response(msg, "NULL", generated::communication::Result::ERR)
    };

    send(socket, response, identity).await?;

    Ok(())
}
