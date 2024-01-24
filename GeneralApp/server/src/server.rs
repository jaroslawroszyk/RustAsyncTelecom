use anyhow::{bail, Result};
use async_zmq::{zmq, Context};
use dotenv_codegen::dotenv;
use generated::communication::*;
use protobuf::Message;
use tokio::net::TcpListener;
use zmq::SNDMORE;

#[derive(Clone)]
pub struct Server {
    context: Context,
    socket_address: String,
}

impl Server {
    pub async fn new() -> Result<Self> {
        if !is_port_available(dotenv!("PORT")).await {
            bail!(async_zmq::Error::EADDRINUSE);
        }

        Ok(Server {
            context: Context::new(),
            socket_address: (&dotenv!("IP_ADDRESS")).to_string(),
        })
    }

    pub async fn run(&self) -> Result<()> {
        let socket = self.context.socket(zmq::ROUTER)?;
        socket.bind(&self.socket_address)?;

        println!("Server is running and waiting for messages...");

        loop {
            let identity: Vec<u8> = socket.recv_msg(0)?.to_vec();
            let message: Vec<u8> = socket.recv_msg(0)?.to_vec();

            match OperationMessage::parse_from_bytes(&message) {
                Ok(msg) => {
                    match msg.msgtype {
                        Some(operation_message::Msgtype::HeartbeatReq(ref msg)) => {
                            println!("Received message: HeartbeatReq {{{msg}}}");
                            let heartbeat_msg_response = build_heartbeat_response();
                            let serialized_heartbeat_msg_response =
                                serialize_message(&heartbeat_msg_response);
                            // println!("jarek identity.clone() heartbeat {:?}", identity.clone()); payload

                            socket.send(&identity, SNDMORE).unwrap();
                            socket.send(serialized_heartbeat_msg_response, 0)?;
                            println!("sent response for hearbeat");
                        }
                        Some(operation_message::Msgtype::AddUserReq(ref msg)) => {
                            println!("Received message: add_user {{{msg}}}");
                            let build_add_user_resp = build_add_user_response(msg);
                            let serialized_build_add_user_resp =
                                serialize_message(&build_add_user_resp);
                            println!("Send to the client message: add_user_resp {{{build_add_user_resp}}}");
                            // println!("jarek identity.clone() AddUserReq {:?}", identity.clone()); payload

                            // tokio::time::sleep(Duration::from_millis(3)).await;
                            socket.send(&identity, SNDMORE).unwrap();
                            socket.send(serialized_build_add_user_resp, 0)?;
                        }
                        Some(operation_message::Msgtype::FooReq(ref msg)) => {
                            println!("Received message: FooReq {{{msg}}}");
                            let build_foo_response = build_foo_response(msg);
                            let serialized_build_foo_response =
                                serialize_message(&build_foo_response);
                            println!("Send to the client message: add_user_resp {{{build_foo_response}}}");
                            socket.send(&identity, SNDMORE).unwrap();
                            socket.send(serialized_build_foo_response, 0)?;
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
    let addres = dotenv!("ADDRESS");
    TcpListener::bind(format!("{addres}:{port}")).await.is_ok()
}

fn serialize_message(msg: &OperationMessage) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    msg.write_to_vec(&mut buf)
        .expect("Failed to serialize message");
    buf
}

fn build_heartbeat_response() -> OperationMessage {
    let mut msg = OperationMessage::new();
    let req = msg.mut_HeartbeatResp();
    req.message = "I'M ALIVE!".into();

    msg
}

fn build_add_user_response(add_user_req: &AddUserReq) -> OperationMessage {
    let mut msg = OperationMessage::new();
    let req = msg.mut_add_user_resp();
    req.user_id = add_user_req.user_id;
    req.user_name = format!("OK RECEIVED for {}", add_user_req.user_name);

    msg
}

fn build_foo_response(foo_req: &FooReq) -> OperationMessage {
    let mut msg = OperationMessage::new();
    let req = msg.mut_foo_resp();
    req.user_name = format!("OK RECEIVED for {}", foo_req.user_name);

    msg
}

pub async fn run_server() -> Result<()> {
    let server = Server::new().await?;
    server.run().await?;
    Ok(())
}
