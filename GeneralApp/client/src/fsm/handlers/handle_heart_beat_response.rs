use std::fmt;

use anyhow::{bail, Result};
use async_zmq::zmq::{self, POLLIN};
use generated::communication::*;
use protobuf::Message;

#[derive(Debug, Clone)]
struct HeartBeatException {}

impl fmt::Display for HeartBeatException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HeartBeatException occurs")
    }
}

// // tak to ma wygladac:
// async fn foo(socket: &zmq::Socket) -> Result<(), HeartBeatException>
// {
//     if socket.poll(POLLIN, 10) != Ok(0) {
//         let Ok(resp) = socket.recv_msg(0) else {
//             return Err(HeartBeatException{});
//         };

//         match Envelope::parse_from_bytes(&resp) {
//             Ok(msg) => match msg.msgtype {
//                 Some(envelope::Msgtype::HeartbeatResp(_)) => {
//                     log::debug!("Received HeartbeatResp from the server {{{msg}}}");
//                 }
//                 _ => {
//                     log::info!("Received unexpected response: {:?}", msg);
//                 }
//             },
//             Err(e) => {
//                 log::warn!("Unable to deserialize response: {:?}", e);
//             }
//         }
//     }
//     else
//     {
//         log::error!("HEARTBEAT DUPA!");
//         // return Err(anyhow!("HEARTBEAT DUPA ANYHOW")); //todo stworzyc customowy error
//         return Err(HeartBeatException{});
//         // bail!("HEARTBEAT DUPA!");
//     }

//     Ok(())
// }

pub async fn handle_heart_beat_response(socket: &zmq::Socket) -> Result<()> {
    //todo implement custom erros
    let mut retries: i8 = 3;

    while retries > 0 {
        if socket.poll(POLLIN, 1000)? != 0 {
            let resp = socket.recv_msg(0)?;

            match Envelope::parse_from_bytes(&resp) {
                Ok(msg) => match msg.msgtype {
                    Some(envelope::Msgtype::HeartbeatResp(_)) => {
                        log::debug!("Received HeartbeatResp from the server {{{msg}}}");
                        return Ok(());
                    }
                    _ => {
                        log::info!("Received unexpected response: {:?}", msg);
                    }
                },
                Err(e) => {
                    log::warn!("Unable to deserialize response: {:?}", e);
                }
            }
        }

        retries -= 1;
        log::info!("Number of retries left: {}", retries);
    }

    bail!((HeartBeatException {}));
}
