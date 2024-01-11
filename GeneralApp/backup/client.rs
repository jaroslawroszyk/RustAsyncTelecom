use async_zmq::{zmq, Context, Result};
use generated::company::*;
use protobuf::Message;
use tokio::time::{Duration, Instant};

const PORT: &str = "5556";

// fn build_message(user_id: u32) -> SomeMsg {
//     let mut msg = SomeMsg::new();
//     let req = msg.mut_add_user_req();

//     req.user_id = user_id;
//     req.user_name = "panicName".into();

//     msg
// }

// fn serialize_message(msg: &SomeMsg) -> Vec<u8> {
//     let mut buf: Vec<u8> = Vec::new();
//     msg.write_to_vec(&mut buf)
//         .expect("Failed to serialize message");
//     buf
// }

// fn generate_messages() -> Vec<SomeMsg> {
//     // let user_ids = 69..80;
//     let user_ids = 69..70;
//     user_ids.map(build_message).collect()
// }

// pub async fn run_client() -> Result<()> {
//     let context = Context::new();
//     let socket = context.socket(zmq::REQ)?;

//     let address = format!("tcp://127.0.0.1:{PORT}");

//     if let Err(e) = socket.connect(&address) {
//         eprintln!("No connection to the server. Cannot send messages. ERR: {:?}", e);
//         return Err(e.into());
//     }

//     // Without delay, the first message will be sent before the connection is established and the message will be lost
//     tokio::time::sleep(Duration::from_millis(1)).await;

//     let heartbeat_msg = build_heartbeat_req_message();
//     let serialized_heartbeat_msg = serialize_message(&heartbeat_msg);

//     if let Err(e) = socket.send(&serialized_heartbeat_msg, 0) {
//         eprintln!("Failed to send HeartbeatReq message. ERR: {:?}", e);
//         return Err(e.into());
//     }

//     let response = receive_response(&socket).await?;
//     match SomeMsg::parse_from_bytes(&response) {
//         Ok(msg) => match msg.msgtype {
//             Some(some_msg::Msgtype::HeartbeatResp(_)) => {
//                 println!("Received HeartbeatResp from the server {{{msg}}}");

//                 for message in &generate_messages() {
//                     println!("Sent message: {message}");
//                     let serialized_msg = serialize_message(message);
                    
//                     // if let Err(e) = socket.send(&serialized_msg, 0) {
//                         // eprintln!("Failed to send message. ERR: {:?}", e);
//                         // return Err(e.into());
//                     // }
//                     let _ = socket.send(&serialized_msg, 0);

//                     tokio::time::sleep(Duration::from_millis(10)).await;
//                 }
//             }
//             Some(some_msg::Msgtype::AddUserResp(_)) => {
//                 println!("Received AddUserResp from the server {{{msg}}}");
//             }
//             _ => {
//                 eprintln!("Received unexpected response: {:?}", msg);
//             }
//         },
//         Err(e) => {
//             eprintln!("Unable to deserialize response: {:?}", e);
//         }
//     }

//     Ok(())
// }

// fn build_heartbeat_req_message() -> SomeMsg {
//     let mut msg = SomeMsg::new();
//     let req = msg.mut_HeartbeatReq();

//     req.message = "YOU ALIVE?".into();

//     msg
// }

// async fn receive_response(socket: &zmq::Socket) -> Result<Vec<u8>> {
//     let message = socket.recv_msg(0)?;
//     Ok(message.to_vec())
// }