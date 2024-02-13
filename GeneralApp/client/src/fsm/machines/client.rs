use anyhow::Result;
use async_zmq::zmq;
use dotenv_codegen::dotenv;
use std::process::exit;
use std::time::Duration;

use crate::fsm::handlers::{
    handle_add_user_response, handle_delete_user_response, handle_exit, handle_heart_beat_response,
    handle_system_time_response, handle_user_info_response,
};
use crate::fsm::state::State;
use crate::fsm::{initialize_client, send_delete_user_request, send_user_info_req};
use crate::fsm::{send_heartbeat_request, send_system_time_req, sending_add_user_req};
use crate::msg_builder::generate_messages;

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
                send_heartbeat_request(&socket).await?;
                tokio::time::sleep(Duration::from_millis(3)).await;

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
                TODO: zapisanie tych uzytkownikow po stronie serwera:
                - zapisac po stronie serwera w vectorze tych userow (do jakiegos structa czy cos)
                - dodac wiadomosc ktora usunie paru userow (DeleteUserReq/Resp)
                - przy konczeniu ma usunac wszystkich userow
                -

                Jak bede mial tych userow to klient bedzie mogl prosic o losowego id w wiadomosci SendingUserInfoRequest.
                a ta wiadomosc powinna powiedziec hej, to user "Jarek" o id "69"
                te requesty o SendingUserInfoRequest powinny byc co jakis czas.
                */
                if let Err(e) = sending_add_user_req(&socket, &mut iter).await {
                    log::error!("Error: {:?}", e);

                    if let Err(e) = socket.disconnect(&dotenv!("IP_ADDRESS")) {
                        log::error!("Error disconnecting socket: {:?}", e);
                    }
                    break;
                }
                state = State::WaitForAddUserResponse;
            }
            State::WaitForAddUserResponse => {
                handle_add_user_response(&socket).await?;
                if iter.peek().is_some() {
                    state = State::SendingAddUserReq;
                } else {
                    state = State::SendingDeleteUserRequest;
                }
            }
            State::SendingDeleteUserRequest => {
                send_delete_user_request(&socket).await?;
                state = State::WaitForDeleteUserResponse;
            }
            State::WaitForDeleteUserResponse => {
                handle_delete_user_response(&socket).await?;
                state = State::SendingUserInfoRequest;
            }
            State::SendingUserInfoRequest => {
                send_user_info_req(&socket).await?;
                tokio::time::sleep(Duration::from_millis(3)).await;
                state = State::WaitForUserInfoResponse;
            }
            State::WaitForUserInfoResponse => {
                handle_user_info_response(&socket).await?;
                state = State::SendSystemTimeReq;
            }
            State::SendSystemTimeReq => {
                send_system_time_req(&socket).await?;
                state = State::WaitForSystemTimeResp;
            }
            State::WaitForSystemTimeResp => {
                handle_system_time_response(&socket).await?;
                state = State::Exit;
            }
            State::Exit => {
                handle_exit(&socket).await?;
                exit(0)
            }
        }
    }
    Ok(())
}

/*
TODO: 1 redis?
1. ADD LOGIN AND AUTHORIZATION CAN THE DATABASE GO?
2. maybe I can add a condition that crashed the server?

TODO: 2 - add proper error handling
*/
