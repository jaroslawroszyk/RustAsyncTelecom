use generated::communication::*;

pub fn build_heartbeat_response() -> OperationMessage {
    let mut msg = OperationMessage::new();
    let req = msg.mut_HeartbeatResp();
    req.message = "I'M ALIVE!".into();

    msg
}
