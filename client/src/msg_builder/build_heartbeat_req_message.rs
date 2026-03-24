use envelope_macro::envelope_builder;
use generated::communication::Envelope;

#[must_use]
#[envelope_builder(mut_HeartbeatReq)]
pub fn build_heartbeat_req_message() -> Envelope {
    inner.message = "HEARTBEAT PING".into();
}
