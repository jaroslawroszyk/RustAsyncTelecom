use generated::company::*;

pub fn build_heartbeat_req_message() -> SomeMsg {
    let mut msg = SomeMsg::new();
    let req = msg.mut_HeartbeatReq();

    req.message = "YOU ALIVE?".into();

    msg
}
