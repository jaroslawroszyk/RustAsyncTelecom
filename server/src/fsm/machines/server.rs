use crate::fsm::states::{
    state_add_user_req, state_delete_user_req, state_heartbeat_req, state_system_time_req,
    state_user_info_req,
};
use anyhow::Result;
use async_zmq::zmq;
use generated::communication::{envelope, Envelope};
use protobuf::Message;
use redis_manager::RedisStateManager;

/// Runs the server state machine that listens for incoming messages from clients and processes them accordingly.
/// It uses a `ZeroMQ` socket to receive messages and a `RedisStateManager` to manage the state of the server in a Redis database.
/// # Errors
/// This function will return an error if it fails to receive a message, deserialize it, or if it fails to process the message based on its type. It will also return an error if it fails to send a response back to the client.
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
                    state_heartbeat_req(socket, msg, redis_state_manager, &identity).await?;
                }
                Some(envelope::Msgtype::AddUserReq(ref msg)) => {
                    state_add_user_req(socket, msg, redis_state_manager, &identity).await?;
                }
                Some(envelope::Msgtype::DeleteUserRequest(ref msg)) => {
                    state_delete_user_req(socket, msg, redis_state_manager, &identity).await?;
                }
                Some(envelope::Msgtype::UserInfoRequest(ref msg)) => {
                    state_user_info_req(socket, msg, redis_state_manager, &identity).await?;
                }
                Some(envelope::Msgtype::SystemTimeReq(_)) => {
                    state_system_time_req(socket, &identity).await?;
                }
                _ => logger::info!("Received unsupported message: {msg}"),
            },
            Err(e) => logger::info!("Unable to deserialize message: {e}"),
        }
    }
}

/*
TODO: wiadomosci response ktore maja: generated::communication::Result::OK powinny zostac nafixowane
*/
