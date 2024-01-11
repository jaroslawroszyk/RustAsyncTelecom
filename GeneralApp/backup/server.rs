// use async_zmq::{zmq, Context, Result};
// use generated::company::*;
// use protobuf::Message;
// use tokio::net::TcpListener;

// const PORT: &str = "5556";

// #[derive(Clone)]
// pub struct Server {
//     context: Context,
//     socket_address: String,
// }

// impl Server {
//     pub async fn new() -> Result<Self> {
//         if !is_port_available(PORT).await {
//             return Err(async_zmq::Error::EADDRINUSE);
//         }

//         Ok(Server {
//             context: Context::new(),
//             socket_address: format!("tcp://127.0.0.1:{PORT}"),
//         })
//     }

//     pub async fn run(&self) -> Result<()> {
//         let socket = self.context.socket(zmq::ROUTER)?;
//         socket.bind(&self.socket_address)?;
//         // socket.set_subscribe(b"")?;

//         println!("Server is running and waiting for messages...");

//         loop {
//             let message: Vec<u8> = read_message(&socket).await?;

//             match SomeMsg::parse_from_bytes(&message) {
//                 Ok(msg) => match msg.msgtype {
//                     Some(some_msg::Msgtype::HeartbeatReq(ref msg)) => {
//                         println!("Received message: HeartbeatReq {{{msg}}}");
//                         let heartbeat_msg_response = build_heartbeat_response();
//                         let serialized_heartbeat_msg_response =
//                             serialize_message(&heartbeat_msg_response);
//                         let client_id: Vec<u8> = socket.recv_msg(0)?.to_vec();
//                         socket.send(client_id, zmq::SNDMORE)?;
//                         socket.send(&serialized_heartbeat_msg_response, 0)?;

//                         // socket.send(&serialized_heartbeat_msg_response, 0)?;
//                     }
//                     Some(some_msg::Msgtype::AddUserReq(ref msg)) => {
//                         println!("Received message: add_user {{{msg}}}");
//                         let build_add_user_resp = build_add_user_response();
//                         let serialized_build_add_user_resp =
//                             serialize_message(&build_add_user_resp);
//                         println!("Send to the client message: add_user_resp {{{build_add_user_resp}}}");
                    
//                         socket.send(&serialized_build_add_user_resp, 0)?;
//                     }
//                     _ => eprintln!("Received unsupported message: {msg}"),
//                 },
//                 Err(e) => eprintln!("Unable to deserialize message: {e}"),
//             }
//         }
//     }
// }

// async fn read_message(socket: &zmq::Socket) -> Result<Vec<u8>> {
//     let message = socket.recv_msg(0)?;
//     Ok(message.to_vec())
// }
