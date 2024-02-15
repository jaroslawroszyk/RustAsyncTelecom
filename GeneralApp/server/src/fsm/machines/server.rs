use anyhow::Result;
use async_zmq::zmq;
use generated::communication::*;
use protobuf::Message;
use redis_manager::RedisStateManager;

use crate::fsm::states::{
    state_add_user_req, state_delete_user_req, state_heartbeat_req, state_system_time_req,
    state_user_info_req,
};

pub async fn run_state_machine(
    socket: &zmq::Socket,
    redis_state_manager: &mut RedisStateManager,
) -> Result<()> {
    loop {
        let identity: Vec<u8> = socket.recv_msg(0)?.to_vec();
        let message: Vec<u8> = socket.recv_msg(0)?.to_vec();

        match Envelope::parse_from_bytes(&message) {
            Ok(msg) => match msg.msgtype {
                Some(envelope::Msgtype::HeartbeatReq(ref msg)) => {
                    state_heartbeat_req(&socket, msg, redis_state_manager, &identity).await?;
                }
                Some(envelope::Msgtype::AddUserReq(ref msg)) => {
                    state_add_user_req(&socket, msg, redis_state_manager, &identity).await?;
                }
                Some(envelope::Msgtype::DeleteUserRequest(ref msg)) => {
                    state_delete_user_req(&socket, msg, redis_state_manager, &identity).await?;
                }
                Some(envelope::Msgtype::UserInfoRequest(ref msg)) => {
                    state_user_info_req(&socket, msg, redis_state_manager, &identity).await?;
                }
                Some(envelope::Msgtype::SystemTimeReq(_)) => {
                    state_system_time_req(&socket, &identity).await?;
                }
                _ => log::info!("Received unsupported message: {msg}"),
            },
            Err(e) => log::info!("Unable to deserialize message: {e}"),
        }
    }
}

/*
TODO: wiadomosci response ktore maja: generated::communication::Result::OK powinny zostac nafixowane
*/
