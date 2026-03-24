use crate::fsm::handlers::{
    handle_add_user_response, handle_delete_user_response, handle_exit, handle_heart_beat_response,
    handle_system_time_response, handle_user_info_response,
};
use crate::fsm::state::State;
use crate::fsm::{initialize_client, send};
use crate::msg_builder::{
    build_delete_user_req, build_heartbeat_req_message, build_system_time_req,
    generate_messages_add_user_req, generate_messages_user_info_req,
};
use anyhow::Result;
use async_zmq::zmq;
use rand::Rng;

/// Runs the client's finite state machine, managing the communication flow with the server.
/// The client goes through several states, including initialization, sending requests, and handling responses.
/// The state machine handles various types of requests such as heartbeat, adding users, deleting users,
/// and requesting user information, as well as system time requests. The client will exit gracefully upon completion or in case of errors.
/// # Errors
/// This function will return an error if any of the operations within the state machine fail, such as sending messages, receiving responses, or processing the responses.
pub async fn run_state_machine(socket: &zmq::Socket) -> Result<()> {
    let mut state = State::Initializing;
    let user_ids = 1..5;
    let add_user_req_msg = generate_messages_add_user_req(user_ids.clone());
    let user_info_req_msg = generate_messages_user_info_req(user_ids.clone());
    let mut iter_add_user_req = add_user_req_msg.iter().peekable();
    let mut iter_user_info_req = user_info_req_msg.iter().peekable();

    loop {
        match state {
            State::Initializing => match initialize_client(socket).await {
                Ok(()) => state = State::SendingHeartbeatReq,
                Err(e) => {
                    logger::error!("{:?}", e);
                    state = State::Exit;
                }
            },
            State::SendingHeartbeatReq => {
                send(socket, &build_heartbeat_req_message()).await?;
                state = State::WaitForHeartBeatResponse;
            }
            State::WaitForHeartBeatResponse => match handle_heart_beat_response(socket).await {
                Ok(()) => state = State::SendingAddUserReq,
                Err(e) => {
                    logger::error!("{:?}", e);
                    state = State::Exit;
                }
            },
            State::SendingAddUserReq => {
                state = if let Some(msg) = iter_add_user_req.next() {
                    if send(socket, msg).await.is_err() {
                        State::Exit
                    } else {
                        State::WaitForAddUserResponse
                    }
                } else {
                    State::SendingDeleteUserRequest
                }
            }
            State::WaitForAddUserResponse => match handle_add_user_response(socket).await {
                Ok(()) => state = State::SendingAddUserReq,
                Err(e) => {
                    logger::error!("{:?}", e);
                    state = State::Exit;
                }
            },
            State::SendingDeleteUserRequest => {
                let mut rng = rand::thread_rng();
                let random_ue = rng.gen_range(user_ids.clone());

                send(socket, &build_delete_user_req(random_ue)).await?;
                state = State::WaitForDeleteUserResponse;
            }
            State::WaitForDeleteUserResponse => match handle_delete_user_response(socket).await {
                Ok(()) => state = State::SendingUserInfoRequest,
                Err(e) => {
                    logger::error!("{:?}", e);
                    state = State::Exit;
                }
            },
            State::SendingUserInfoRequest => {
                state = if let Some(msg) = iter_user_info_req.next() {
                    if send(socket, msg).await.is_err() {
                        State::Exit
                    } else {
                        State::WaitForUserInfoResponse
                    }
                } else {
                    State::SendSystemTimeReq
                }
            }
            State::WaitForUserInfoResponse => match handle_user_info_response(socket).await {
                Ok(()) => state = State::SendingUserInfoRequest,
                Err(e) => {
                    logger::error!("{:?}", e);
                    state = State::Exit;
                }
            },
            State::SendSystemTimeReq => {
                send(socket, &build_system_time_req()).await?;
                state = State::WaitForSystemTimeResp;
            }
            State::WaitForSystemTimeResp => match handle_system_time_response(socket).await {
                Ok(()) => state = State::Exit,
                Err(e) => {
                    logger::error!("{:?}", e);
                    state = State::Exit;
                }
            },
            State::Exit => {
                handle_exit(socket).await?;
                break;
            }
        }
    }
    Ok(())
}
