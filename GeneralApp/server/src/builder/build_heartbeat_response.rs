use generated::communication::*;

pub fn build_heartbeat_response() -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_HeartbeatResp();
    req.message = "HEARTBEAT PONG".into();

    msg
}
