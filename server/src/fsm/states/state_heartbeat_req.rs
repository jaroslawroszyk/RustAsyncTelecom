use crate::{builder::build_heartbeat_response, fsm::machines::send};
use anyhow::Result;
use async_zmq::zmq;
use generated::communication::HeartbeatReq;
use redis_manager::{namespace::HEARTBEAT_NS, RedisStateManager};

/// Handles the `HeartbeatReq` message received from the client.
/// It increments a heartbeat counter in the Redis database and sends back a response indicating success or failure
/// # Errors
/// This function will return an error if it fails to increment the heartbeat counter in the Redis database or if it fails to send the response back to the client.
pub async fn state_heartbeat_req(
    socket: &zmq::Socket,
    msg: &HeartbeatReq,
    redis_state_manager: &mut RedisStateManager,
    identity: &[u8],
) -> Result<()> {
    logger::debug!("Received message: HeartbeatReq {{{msg}}}");

    let response = if redis_state_manager
        .increment_counter(HEARTBEAT_NS)
        .await
        .is_ok()
    {
        let current = redis_state_manager.get_counter(HEARTBEAT_NS).await?;
        logger::debug!("Counter HEARTBEAT_NS : {current}");

        build_heartbeat_response(generated::communication::Result::OK)
    } else {
        logger::error!("Redis increment failed for heartbeat");
        build_heartbeat_response(generated::communication::Result::ERR)
    };

    // CRITICAL: You MUST await the send call, otherwise it's a "floating future"
    send(socket, response, identity).await?;

    Ok(())
}
