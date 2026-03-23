use anyhow::Result;
use async_zmq::zmq;
use generated::communication::DeleteUserRequest;
use redis_manager::{namespace::USERS_NS, RedisStateManager};

use crate::{builder::build_delete_user_response, fsm::machines::send};

pub async fn state_delete_user_req(
    socket: &zmq::Socket,
    msg: &DeleteUserRequest,
    redis_state_manager: &mut RedisStateManager,
    identity: &[u8],
) -> Result<()> {
    log::debug!("Received message: DeleteUserRequest {{{msg}}}");

    log::debug!(
        "DB STATE before remove user: {} name: {:?}",
        msg.user_id,
        redis_state_manager.get_all_from_ns(USERS_NS).await?
    );

    let delete_user_id = msg.user_id;

    let get_from_redis = redis_state_manager
        .get(USERS_NS, &delete_user_id.to_string())
        .await;
    let result_reids_call = redis_state_manager
        .delete(USERS_NS, &delete_user_id.to_string())
        .await;
    let response = match result_reids_call {
        Ok(_) => {
            let user_name = get_from_redis?;
            log::debug!(
                "Delete user: {} name :{} from db",
                delete_user_id,
                user_name
            );
            build_delete_user_response(msg, &user_name, generated::communication::Result::OK)
        }
        Err(_) => {
            log::error!("nie udalo sie i co mi zrobisz?");
            build_delete_user_response(msg, "NULL", generated::communication::Result::ERR)
        }
    };

    _ = send(socket, response, identity);

    log::debug!(
        "Db state after REMOVE user: {} name: {:?}",
        msg.user_id,
        redis_state_manager.get_all_from_ns(USERS_NS).await?
    );
    Ok(())
}
