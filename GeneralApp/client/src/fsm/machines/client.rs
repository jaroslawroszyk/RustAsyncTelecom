use std::time::Duration;

use anyhow::{Ok, Result};
use async_zmq::zmq;
use dotenv_codegen::dotenv;
use generated::company::*;

use crate::fsm::initialize_client;
use crate::fsm::state::State;
use crate::fsm::{send_heartbeat_request, sending_add_user_req};

fn build_message(user_id: u32, user_name: &str) -> SomeMsg {
    let mut msg = SomeMsg::new();
    let req = msg.mut_add_user_req();

    req.user_id = user_id;
    req.user_name = user_name.to_string();

    msg
}

fn generate_messages() -> Vec<SomeMsg> {
    let user_ids = 1..=10;
    let user_names = vec![
        "Alice", "Bob", "Charlie", "David", "Eva", "Frank", "Grace", "Henry", "Ivy", "Jack",
    ];

    user_ids
        .zip(user_names.into_iter())
        .map(|(id, name)| build_message(id, name))
        .collect()
}

pub async fn run_state_machine(socket: &zmq::Socket) -> Result<()> {
    let mut state = State::Initializing;
    let messages = generate_messages();
    let mut iter = messages.iter();

    loop {
        match state {
            State::Initializing => {
                initialize_client(&socket).await?;
                state = State::SendingHeartbeatReq;
            }
            State::SendingHeartbeatReq => {
                send_heartbeat_request(&socket).await?;
                tokio::time::sleep(Duration::from_millis(3)).await;

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
            }
        }
    }
    Ok(())
}
