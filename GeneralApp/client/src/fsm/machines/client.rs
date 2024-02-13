use anyhow::Result;
use async_zmq::zmq;
use std::time::Duration;

use crate::fsm::handlers::{
    handle_add_user_response, handle_delete_user_response, handle_exit, handle_heart_beat_response,
    handle_system_time_response, handle_user_info_response,
};
use crate::fsm::state::State;
use crate::fsm::{initialize_client, send};
use crate::msg_builder::{
    build_delete_user_req, build_heartbeat_req_message, build_system_time_req, build_user_info_req,
    generate_messages,
};

pub async fn run_state_machine(socket: &zmq::Socket) -> Result<()> {
    let mut state = State::Initializing;
    let messages = generate_messages();
    let mut iter = messages.iter().peekable();

    loop {
        match state {
            State::Initializing => {
                initialize_client(&socket).await?;
                state = State::SendingHeartbeatReq;
            }
            State::SendingHeartbeatReq => {
                send(&socket, &build_heartbeat_req_message()).await?;

                state = State::WaitForHeartBeatResponse;
            }
            State::WaitForHeartBeatResponse => match handle_heart_beat_response(&socket).await {
                Ok(_) => state = State::SendingAddUserReq,
                Err(e) => {
                    log::error!("{:?}", e);
                    state = State::Exit;
                }
            },
            State::SendingAddUserReq => {
                /*
                TODO:
                Jak bede mial tych userow to klient bedzie mogl prosic o losowego id w wiadomosci SendingUserInfoRequest.
                a ta wiadomosc powinna powiedziec hej, to user "Jarek" o id "69"
                te requesty o SendingUserInfoRequest powinny byc co jakis czas.
                */
                state = if let Some(msg) = iter.next() {
                    if send(&socket, msg).await.is_err() {
                        State::Exit
                    } else {
                        State::WaitForAddUserResponse
                    }
                } else {
                    State::SendingDeleteUserRequest
                }
            }
            State::WaitForAddUserResponse => {
                handle_add_user_response(&socket).await?;
                state = State::SendingAddUserReq;
            }
            State::SendingDeleteUserRequest => {
                send(&socket, &build_delete_user_req()).await?;
                state = State::WaitForDeleteUserResponse;
            }
            State::WaitForDeleteUserResponse => {
                handle_delete_user_response(&socket).await?;
                state = State::SendingUserInfoRequest;
            }
            State::SendingUserInfoRequest => {
                send(&socket, &build_user_info_req()).await?;

                tokio::time::sleep(Duration::from_millis(3)).await;
                state = State::WaitForUserInfoResponse;
            }
            State::WaitForUserInfoResponse => {
                handle_user_info_response(&socket).await?;
                state = State::SendSystemTimeReq;
            }
            State::SendSystemTimeReq => {
                send(&socket, &build_system_time_req()).await?;
                state = State::WaitForSystemTimeResp;
            }
            State::WaitForSystemTimeResp => {
                handle_system_time_response(&socket).await?;
                state = State::Exit;
            }
            State::Exit => {
                handle_exit(&socket).await?;
                break;
            }
        }
    }
    Ok(())
}

/*
TODO: 2 - add proper error handling
*/
