use std::time::Duration;

use anyhow::Result;
use async_zmq::zmq::{self, POLLIN};
use dotenv_codegen::dotenv;
use generated::communication::*;
use protobuf::Message;

use crate::fsm::initialize_client;
use crate::fsm::state::State;
use crate::fsm::{send_foo_req, send_heartbeat_request, sending_add_user_req};

fn build_message_add_user_req(user_id: u32, user_name: &str) -> OperationMessage {
    let mut msg = OperationMessage::new();
    let req: &mut AddUserReq = msg.mut_add_user_req();

    req.user_id = user_id;
    req.user_name = user_name.to_string();

    msg
}

fn generate_messages() -> Vec<OperationMessage> {
    let user_ids = 1..=3;
    let user_names = vec![
        "Alice", "Bob", "Charlie", "David", "Eva", "Frank", "Grace", "Henry", "Ivy", "Jack",
    ];

    user_ids
        .zip(user_names.into_iter())
        .map(|(id, name)| build_message_add_user_req(id, name))
        .collect()
}
/*
pub async fn receive_response(socket: &zmq::Socket) -> Result<()> {
    if socket.poll(POLLIN, 10)? != 0 {
        let resp = socket.recv_msg(0)?;

        match OperationMessage::parse_from_bytes(&resp) {
            Ok(msg) => match msg.msgtype {
                Some(operation_message::Msgtype::HeartbeatResp(_)) => {
                    println!("Received HeartbeatResp from the server {{{msg}}}");
                }
                Some(operation_message::Msgtype::AddUserResp(_)) => {
                    println!("Received AddUserResp from the server {{{msg}}}");
                }
                Some(operation_message::Msgtype::FooResp(_)) => {
                    println!("Received FooResp from the server {{{msg}}}");
                }
                _ => {
                    eprintln!("Received unexpected response: {:?}", msg);
                }
            },
            Err(e) => {
                eprintln!("Unable to deserialize response: {:?}", e);
            }
        }
    }

    Ok(())
}

*/

async fn handle_heart_beat_response(socket: &zmq::Socket) -> Result<()> {
    if socket.poll(POLLIN, 10)? != 0 {
        let resp = socket.recv_msg(0)?;

        match OperationMessage::parse_from_bytes(&resp) {
            Ok(msg) => match msg.msgtype {
                Some(operation_message::Msgtype::HeartbeatResp(_)) => {
                    println!("Received HeartbeatResp from the server {{{msg}}}");
                }
                _ => {
                    eprintln!("Received unexpected response: {:?}", msg);
                }
            },
            Err(e) => {
                eprintln!("Unable to deserialize response: {:?}", e);
            }
        }
    }

    Ok(())
}

async fn handle_add_user_response(socket: &zmq::Socket) -> Result<()> {
    if socket.poll(POLLIN, 10)? != 0 {
        let resp = socket.recv_msg(0)?;

        match OperationMessage::parse_from_bytes(&resp) {
            Ok(msg) => match msg.msgtype {
                Some(operation_message::Msgtype::AddUserResp(_)) => {
                    println!("Received AddUserResp from the server {{{msg}}}");
                }
                _ => {
                    eprintln!("Received unexpected response: {:?}", msg);
                }
            },
            Err(e) => {
                eprintln!("Unable to deserialize response: {:?}", e);
            }
        }
    }

    Ok(())
}

async fn handle_foo_response(socket: &zmq::Socket) -> Result<()> {
    if socket.poll(POLLIN, 10)? != 0 {
        let resp = socket.recv_msg(0)?;

        match OperationMessage::parse_from_bytes(&resp) {
            Ok(msg) => match msg.msgtype {
                Some(operation_message::Msgtype::FooResp(_)) => {
                    println!("Received FooResp from the server {{{msg}}}");
                }
                _ => {
                    eprintln!("Received unexpected response: {:?}", msg);
                }
            },
            Err(e) => {
                eprintln!("Unable to deserialize response: {:?}", e);
            }
        }
    }

    Ok(())
}

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

                break;
            } //todo state done bye client!
        }
    }
    Ok(())
}
/*
TODO DODAC LOGOWANIE AUTHORYZACJE MOZE BAZA DANYCH WLECI?
*/
