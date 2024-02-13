use anyhow::Result;
use async_zmq::zmq;
use generated::communication::*;
use protobuf::Message;
use redis_manager::{RedisStateManager, HEARTBEAT_NS, USERS_NS};
use zmq::SNDMORE;

use crate::{
    builder::{
        build_add_user_response, build_heartbeat_response, build_system_time_response,
        build_user_info_response,
    },
    serializers::serialize_message,
    // server::RedisStateManager,
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
                    // TODO: add an id to hearbeat and store these ids in redis
                    log::debug!("Received message: HeartbeatReq {{{msg}}}");

                    let serialized_heartbeat_msg_response =
                        serialize_message(&build_heartbeat_response());

                    redis_state_manager.increment_counter(HEARTBEAT_NS).await?;
                    socket.send(&identity, SNDMORE).unwrap();
                    socket.send(serialized_heartbeat_msg_response, 0)?;
                    // redis_state_manager.reset_counter(HEARTBEAT_NS).await?;
                    // log::info!("jarek HeartbeatCounter: {}", redis_state_manager.get_counter("HeartbeatReqCounter").await?); //to delete
                }
                Some(envelope::Msgtype::AddUserReq(ref msg)) => {
                    log::debug!("Received message: AddUserRequest {{{msg}}}");

                    let build_add_user_resp = build_add_user_response(msg);
                    let serialized_build_add_user_resp = serialize_message(&build_add_user_resp);
                    println!(
                        "Send to the client message: AddUserResponse {{{build_add_user_resp}}}"
                    );

                    redis_state_manager
                        .set(USERS_NS, &msg.user_id.to_string(), &msg.user_name)
                        .await?;

                    socket.send(&identity, SNDMORE).unwrap();
                    socket.send(serialized_build_add_user_resp.clone(), 0)?;

                    log::info!(
                        "Value for userId: {} is: {}",
                        msg.user_id,
                        redis_state_manager
                            .get(USERS_NS, &msg.user_id.to_string())
                            .await?
                    );
                }
                Some(envelope::Msgtype::UserInfoRequest(ref msg)) => {
                    // TODO: read things from redis and send what the client requested
                    log::debug!("Received message: UserInfoRequest {{{msg}}}");

                    let build_user_info_response = build_user_info_response(msg);
                    let serialized_build_build_user_info_response =
                        serialize_message(&build_user_info_response);

                    println!("Send to the client message: UserInfoRequest {{{build_user_info_response}}}");
                    socket.send(&identity, SNDMORE).unwrap();
                    socket.send(serialized_build_build_user_info_response, 0)?;
                }
                Some(envelope::Msgtype::SystemTimeReq(_)) => {
                    log::debug!("Received message: SystemTimeRequest");

                    let build_system_time_response = build_system_time_response();
                    let serialized_build_system_time_response =
                        serialize_message(&build_system_time_response);
                    println!("Send to the client message: UserInfoResponse {{{build_system_time_response}}}");
                    socket.send(&identity, SNDMORE).unwrap();
                    socket.send(serialized_build_system_time_response, 0)?;
                }
                _ => log::info!("Received unsupported message: {msg}"),
            },
            Err(e) => log::info!("Unable to deserialize message: {e}"),
        }
    }
}
