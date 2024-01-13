use std::time::Duration;

use anyhow::{bail, Result};
use async_zmq::{zmq, Context};
use generated::company::*;
use protobuf::Message;
use tokio::net::TcpListener;
use zmq::SNDMORE;

const PORT: &str = "5556";

#[derive(Clone)]
pub struct Server {
    context: Context,
    socket_address: String,
}

impl Server {
    pub async fn new() -> Result<Self> {
        if !is_port_available(PORT).await {
            bail!(async_zmq::Error::EADDRINUSE);
        }

        Ok(Server {
            context: Context::new(),
            socket_address: format!("tcp://127.0.0.1:{}", PORT),
        })
    }

    pub async fn run(&self) -> Result<()> {
        let socket = self.context.socket(zmq::ROUTER)?;
        socket.bind(&self.socket_address)?;

        println!("Server is running and waiting for messages...");

        loop {
            let identity: Vec<u8> = socket.recv_msg(0)?.to_vec();
            let message: Vec<u8> = socket.recv_msg(0)?.to_vec();

            match SomeMsg::parse_from_bytes(&message) {
                Ok(msg) => {
                    match msg.msgtype {
                        Some(some_msg::Msgtype::HeartbeatReq(ref msg)) => {
                            println!("Received message: HeartbeatReq {{{msg}}}");
                            let heartbeat_msg_response = build_heartbeat_response();
                            let serialized_heartbeat_msg_response =
                                serialize_message(&heartbeat_msg_response);
                            println!("jarek identity.clone() heartbeat {:?}", identity.clone());

                            socket.send(&identity, SNDMORE).unwrap();
                            socket.send(serialized_heartbeat_msg_response, 0)?;
                            println!("sent response for hearbeat");
                        }
                        Some(some_msg::Msgtype::AddUserReq(ref msg)) => {
                            println!("Received message: add_user {{{msg}}}");
                            let build_add_user_resp = build_add_user_response();
                            let serialized_build_add_user_resp =
                                serialize_message(&build_add_user_resp);
                            println!("Send to the client message: add_user_resp {{{build_add_user_resp}}}");
                            println!("jarek identity.clone() AddUserReq {:?}", identity.clone());

                            tokio::time::sleep(Duration::from_millis(3)).await;
                            socket.send(&identity, SNDMORE).unwrap();
                            socket.send(serialized_build_add_user_resp, 0)?;
                        }
                        _ => eprintln!("Received unsupported message: {msg}"),
                    }
                }
                Err(e) => eprintln!("Unable to deserialize message: {e}"),
            }
        }
    }
}

async fn is_port_available(port: &str) -> bool {
    TcpListener::bind(format!("127.0.0.1:{port}")).await.is_ok()
}

fn serialize_message(msg: &SomeMsg) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    msg.write_to_vec(&mut buf)
        .expect("Failed to serialize message");
    buf
}

//TODO: change message to the client
fn build_heartbeat_response() -> SomeMsg {
    let mut msg = SomeMsg::new();
    let req = msg.mut_HeartbeatResp();
    req.message = "I'M ALIVE!".into();

    msg
}

//TODO: read user_id and user_name from request
fn build_add_user_response() -> SomeMsg {
    let mut msg = SomeMsg::new();
    let req = msg.mut_add_user_resp();
    req.user_id = 420;
    req.user_name = "OK RECEIVED".into();

    msg
}

pub async fn run_server() -> Result<()> {
    let server = Server::new().await?;
    server.run().await?;
    Ok(())
}
