use anyhow::Result;
use async_zmq::zmq;
use generated::communication::*;
use protobuf::Message;
use redis_manager::{namespace::*, RedisStateManager};

use crate::{
    builder::{
        build_add_user_response, build_delete_user_response, build_heartbeat_response,
        build_system_time_response, build_user_info_response,
    },
    fsm::machines::send,
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
                // TODO: add an id to hearbeat and store these ids in redis
                Some(envelope::Msgtype::HeartbeatReq(ref msg)) => {
                    log::debug!("Received message: HeartbeatReq {{{msg}}}");

                    let result_redis_call =
                        redis_state_manager.increment_counter(HEARTBEAT_NS).await;
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

                    _ = send(&socket, response, &identity);
                }
                Some(envelope::Msgtype::AddUserReq(ref msg)) => {
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
                }
                Some(envelope::Msgtype::DeleteUserRequest(ref msg)) => {
                    log::debug!("Received message: DeleteUserRequest {{{msg}}}");

                    log::debug!(
                        "DB STATE before remove user: {} name: {:?}",
                        msg.user_id,
                        redis_state_manager.get_all_from_ns(USERS_NS).await?
                    );

                    let delete_user_id = msg.user_id;
                    let delete_user_name = &msg.username;
                    let result_reids_call = redis_state_manager
                        .delete(USERS_NS, &delete_user_id.to_string())
                        .await;
                    let response = match result_reids_call {
                        Ok(_) => {
                            log::debug!(
                                "Delete user: {} name :{} from db",
                                delete_user_id,
                                delete_user_name
                            );
                            build_delete_user_response(msg, generated::communication::Result::OK)
                        }
                        Err(_) => {
                            log::error!("nie udalo sie i co mi zrobisz?");
                            build_delete_user_response(msg, generated::communication::Result::ERR)
                        }
                    };

                    _ = send(&socket, response, &identity);

                    log::debug!(
                        "Db state after REMOVE user: {} name: {:?}",
                        msg.user_id,
                        redis_state_manager.get_all_from_ns(USERS_NS).await?
                    );
                }
                Some(envelope::Msgtype::UserInfoRequest(ref msg)) => {
                    // TODO: read things from redis and send what the client requested
                    log::debug!("Received message: UserInfoRequest {{{msg}}}");
                    let username_from_db: String = redis_state_manager
                        .get(USERS_NS, &msg.user_id.to_string())
                        .await?;
                    _ = send(
                        &socket,
                        build_user_info_response(
                            msg,
                            username_from_db,
                            generated::communication::Result::OK,
                        ),
                        &identity,
                    );
                }
                Some(envelope::Msgtype::SystemTimeReq(_)) => {
                    log::debug!("Received message: SystemTimeRequest");

                    _ = send(
                        &socket,
                        build_system_time_response(generated::communication::Result::OK),
                        &identity,
                    );
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
