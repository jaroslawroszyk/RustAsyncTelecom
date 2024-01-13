use anyhow::Result;
use async_zmq::{
    zmq::{self, POLLIN},
    Context,
};
use generated::company::*;
use lazy_static::lazy_static;
use protobuf::Message;
use rand::Rng;
use std::time::Duration;

//TODO: czytanie portu i addresu zrob z pliku config :)
const PORT: &str = "5556";

lazy_static! {
    static ref ADDRESS: String = format!("tcp://127.0.0.1:{}", PORT);
}

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

fn serialize_message(msg: &SomeMsg) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    msg.write_to_vec(&mut buf)
        .expect("Failed to serialize message");
    buf
}

fn generate_messages() -> Vec<SomeMsg> {
    let user_ids = 69..=78;
    user_ids.map(build_message).collect()
}

fn build_heartbeat_req_message() -> SomeMsg {
    let mut msg = SomeMsg::new();
    let req = msg.mut_HeartbeatReq();

    req.message = "YOU ALIVE?".into();

    msg
}

//TODO: extract these functions to state file
async fn initialize_client(socket: &zmq::Socket) -> Result<()> {
    let mut rng = rand::thread_rng();
    let client_id: String = rng.gen_range(1000..9999).to_string();

    socket.set_identity(client_id.as_bytes())?;
    match socket.connect(&ADDRESS) {
        Err(e) => eprintln!("No connection to the server. Cannot send messages. ERR: {e}"),
        Ok(_) => println!("Connected to the server at tcp://127.0.0.1:{PORT}"),
    };

    Ok(())
}

async fn send_heartbeat_request(socket: &zmq::Socket) -> Result<()> {
    let heartbeat_msg = build_heartbeat_req_message();
    let serialized_heartbeat_msg = serialize_message(&heartbeat_msg);

    if let Err(e) = socket.send(&serialized_heartbeat_msg, 0) {
        eprintln!("Failed to send HeartbeatReq message. ERR: {:?}", e);
        return Err(e.into());
    }

    println!("Sent HeartbeatReq message");
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}

async fn sending_add_user_req(
    socket: &zmq::Socket,
    iter: &mut std::slice::Iter<'_, SomeMsg>,
) -> Result<()> {
    if let Some(message) = iter.next() {
        let serialized_msg = serialize_message(message);

        if let Err(e) = socket.send(&serialized_msg, 0) {
            eprintln!("Failed to send message. ERR: {:?}", e);
            return Err(e.into());
        }

        println!("Sent message: {{{message}}}");
        tokio::time::sleep(Duration::from_secs(1)).await;

        if socket.poll(POLLIN, 10)? != 0 {
            let resp = socket.recv_msg(0)?;

            match SomeMsg::parse_from_bytes(&resp) {
                Ok(msg) => match msg.msgtype {
                    Some(some_msg::Msgtype::AddUserResp(_)) => {
                        println!("Received AddUserResp from the server {{{msg}}}");
                    }
                    Some(some_msg::Msgtype::HeartbeatResp(_)) => {
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
    } else {
        Err(anyhow::anyhow!("No more messages to send"))
    }
}

//state machines method

//TODO: wysylamy heartbet i nie czekamy na response - dodac ze jak response nie przyjdzie nie idzie dalej!
pub async fn run_client() -> Result<()> {
    let context = Context::new();
    let socket = context.socket(zmq::DEALER)?;

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
                state = State::SendingAddUserReq;
            }
            State::SendingAddUserReq => {
                if let Err(e) = sending_add_user_req(&socket, &mut iter).await {
                    eprintln!("Error: {:?}", e);
                    if let Err(e) = socket.disconnect(&ADDRESS) {
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
