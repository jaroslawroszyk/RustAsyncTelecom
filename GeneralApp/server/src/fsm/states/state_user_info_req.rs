use anyhow::Result;
use async_zmq::zmq;
use generated::communication::UserInfoRequest;
use redis_manager::{namespace::USERS_NS, RedisStateManager};

use crate::{builder::build_user_info_response, fsm::machines::send};

pub async fn state_user_info_req(
    socket: &zmq::Socket,
    msg: &UserInfoRequest,
    redis_state_manager: &mut RedisStateManager,
    identity: &[u8],
) -> Result<()> {
    log::debug!("Received message: UserInfoRequest {{{msg}}}");
    let username_from_db = redis_state_manager
        .get(USERS_NS, &msg.user_id.to_string())
        .await;

    let response = match username_from_db {
        Ok(username) => build_user_info_response(username, generated::communication::Result::OK),
        Err(_) => {
            build_user_info_response("NULL".to_string(), generated::communication::Result::ERR)
        }
    };
    _ = send(&socket, response, &identity);
    Ok(())
}
