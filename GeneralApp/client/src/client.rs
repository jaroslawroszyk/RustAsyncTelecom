use anyhow::Result;
use async_zmq::{
    zmq::{self, POLLIN},
    Context,
};
use dotenv_codegen::dotenv;
use generated::company::*;
use protobuf::Message;
use std::time::Duration;

use crate::fsm::{send_heartbeat_request, sending_add_user_req};
use crate::{
    fsm::{initialize_client, machines},
    serializers::serialize_message,
};

enum State {
    Initializing,
    SendingHeartbeatReq,
    SendingAddUserReq,
}

fn build_message(user_id: u32) -> SomeMsg {
    let mut msg = SomeMsg::new();
    let req = msg.mut_add_user_req();

    req.user_id = user_id;
    req.user_name = "panicName".into();

    msg
}

fn generate_messages() -> Vec<SomeMsg> {
    let user_ids = 69..=72;
    user_ids.map(build_message).collect()
}

//TODO: wysylamy heartbet i nie czekamy na response - dodac ze jak response nie przyjdzie nie idzie dalej!
pub async fn run_client() -> Result<()> {
    let context = Context::new();
    let socket = context.socket(zmq::DEALER)?;

    let mut state = State::Initializing;
    let messages = generate_messages();
    let mut iter = messages.iter();
    machines::hello();
    loop {
        match state {
            State::Initializing => {
                initialize_client(&socket).await?;
                state = State::SendingHeartbeatReq;
            }
            State::SendingHeartbeatReq => {
                send_heartbeat_request(&socket).await?;
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

/*
TODO: jezeli nie przyjdzie resposne na heartbeat to zamknij gniazdo! (TIMEOUT?)
*/
