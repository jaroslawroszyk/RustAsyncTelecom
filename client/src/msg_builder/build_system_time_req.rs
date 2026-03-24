use envelope_macro::envelope_builder;
use generated::communication::Envelope;

#[must_use]
#[envelope_builder(mut_system_time_req)]
pub fn build_system_time_req() -> Envelope {
    let _ = inner;
}
