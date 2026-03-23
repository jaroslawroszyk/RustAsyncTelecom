use generated::communication::Envelope;

#[must_use]
pub fn build_heartbeat_req_message() -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_HeartbeatReq();

    req.message = "HEARTBEAT PING".into();

    msg
}
