use generated::communication::Result;
use generated::communication::*;

pub fn build_heartbeat_response(result: Result) -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_HeartbeatResp();

    req.message = "HEARTBEAT PONG".into();
    req.result = result.into();

    msg
}
