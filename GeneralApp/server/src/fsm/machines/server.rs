use anyhow::Result;
use async_zmq::zmq;
use generated::communication::*;
use protobuf::Message;
use zmq::SNDMORE;

use crate::{
    builder::{
        build_add_user_response, build_heartbeat_response, build_system_time_response,
        build_user_info_response,
    },
    serializers::serialize_message,
    server::RedisStateManager,
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
                    log::debug!("Received message: HeartbeatReq {{{msg}}}");

                    let serialized_heartbeat_msg_response =
                        serialize_message(&build_heartbeat_response());
                    socket.send(&identity, SNDMORE).unwrap();
                    socket.send(serialized_heartbeat_msg_response, 0)?;
                }
                Some(envelope::Msgtype::AddUserReq(ref msg)) => {
                    log::debug!("Received message: AddUserRequest {{{msg}}}");

                    let build_add_user_resp = build_add_user_response(msg);
                    let serialized_build_add_user_resp = serialize_message(&build_add_user_resp);
                    println!(
                        "Send to the client message: AddUserResponse {{{build_add_user_resp}}}"
                    );

                    redis_state_manager.increment_counter("AddUserReq").await?;
                    socket.send(&identity, SNDMORE).unwrap();
                    socket.send(serialized_build_add_user_resp.clone(), 0)?;

                    // redis_state_manager.save_state(counter).await?;  Czy zapis stanu powinien byc po wyslaniu requesta?

                    // let add_user_req_counter: i32 = redis_client.get("add_user_req_counter")?; //TODO: decrement user_counter when msg deleteUser arrives
                    log::info!(
                        "Value of add_user_req_counter: {}",
                        redis_state_manager.get_counter("AddUserReq").await?
                    );
                    log::info!(
                        "Value of app_state_ns: {}",
                        redis_state_manager.get_counter("app_state_counter").await?
                    );
                }
                Some(envelope::Msgtype::UserInfoRequest(ref msg)) => {
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
