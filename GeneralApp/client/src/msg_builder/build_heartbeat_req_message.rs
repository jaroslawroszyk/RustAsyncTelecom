use generated::communication::*;

pub fn build_heartbeat_req_message() -> OperationMessage {
    let mut msg = OperationMessage::new();
    let req = msg.mut_HeartbeatReq();

    req.message = "YOU ALIVE?".into();

    msg
}
