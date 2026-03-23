use envelope_macro::envelope_builder;
use generated::communication::Envelope;
use generated::communication::Result;

#[must_use]
#[envelope_builder(mut_HeartbeatResp)]
pub fn build_heartbeat_response(result: Result) -> Envelope {
    inner.message = "HEARTBEAT PONG".into();
    inner.result = result.into();
}
