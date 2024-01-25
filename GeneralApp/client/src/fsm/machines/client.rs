use anyhow::Result;
use async_zmq::zmq;
use dotenv_codegen::dotenv;
use std::time::Duration;

use crate::fsm::handlers::{
    handle_add_user_response, handle_exit, handle_foo_response, handle_heart_beat_response,
};
use crate::fsm::initialize_client;
use crate::fsm::state::State;
use crate::fsm::{send_foo_req, send_heartbeat_request, sending_add_user_req};
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
            State::WaitForHeartBeatResponse => {
                handle_heart_beat_response(&socket).await?;
                state = State::SendingAddUserReq;
            }
            State::SendingAddUserReq => {
                if let Err(e) = sending_add_user_req(&socket, &mut iter).await {
                    eprintln!("Error: {:?}", e);
                    if let Err(e) = socket.disconnect(&dotenv!("IP_ADDRESS")) {
                        eprintln!("Error disconnecting socket: {:?}", e);
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
                    state = State::SendingFooReq;
                }
            }
            State::SendingFooReq => {
                send_foo_req(&socket).await?;
                tokio::time::sleep(Duration::from_millis(3)).await;
                state = State::WaitForFooResponse;
            }
            State::WaitForFooResponse => {
                handle_foo_response(&socket).await?;
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
TODO DODAC LOGOWANIE AUTHORYZACJE MOZE BAZA DANYCH WLECI?
*/
