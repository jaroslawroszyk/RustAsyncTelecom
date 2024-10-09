use anyhow::Result;
use async_zmq::zmq;
use generated::communication::AddUserReq;
use redis_manager::{namespace::USERS_NS, RedisStateManager};

use crate::{builder::build_add_user_response, fsm::machines::send};

pub async fn state_add_user_req(
    socket: &zmq::Socket,
    msg: &AddUserReq,
    redis_state_manager: &mut RedisStateManager,
    identity: &[u8],
) -> Result<()> {
    log::debug!("Received message: AddUserRequest {{{msg}}}");

    _ = send(
        &socket,
        build_add_user_response(msg, generated::communication::Result::OK),
        &identity,
    );

    redis_state_manager
        .set(USERS_NS, &msg.user_id.to_string(), &msg.user_name)
        .await?;

    log::debug!(
        "Value for userId: {} is: {}",
        msg.user_id,
        redis_state_manager
            .get(USERS_NS, &msg.user_id.to_string())
            .await?
    );
    Ok(())
}
