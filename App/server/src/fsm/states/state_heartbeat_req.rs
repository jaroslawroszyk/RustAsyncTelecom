use anyhow::Result;
use async_zmq::zmq;
use generated::communication::HeartbeatReq;
use redis_manager::{namespace::HEARTBEAT_NS, RedisStateManager};

use crate::{builder::build_heartbeat_response, fsm::machines::send};

pub async fn state_heartbeat_req(
    socket: &zmq::Socket,
    msg: &HeartbeatReq,
    redis_state_manager: &mut RedisStateManager,
    identity: &[u8],
) -> Result<()> {
    log::debug!("Received message: HeartbeatReq {{{msg}}}");

    let result_redis_call = redis_state_manager.increment_counter(HEARTBEAT_NS).await;
    let response = match result_redis_call {
        Ok(_) => {
            log::debug!(
                "Counter HEARTBEAT_NS : {}",
                redis_state_manager.get_counter(HEARTBEAT_NS).await?
            );
            build_heartbeat_response(generated::communication::Result::OK)
        }
        Err(_) => {
            log::error!("nie udalo sie i co mi zrobisz?");
            build_heartbeat_response(generated::communication::Result::ERR)
        }
    };

    _ = send(socket, response, identity);
    Ok(())
}
