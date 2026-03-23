use envelope_macro::envelope_builder;
use generated::communication::Envelope;
use std::ops::Range;

#[must_use]
#[envelope_builder(mut_user_info_request)]
pub fn build_user_info_req(user_id: u32) -> Envelope {
    inner.user_id = user_id;
}

pub fn generate_messages_user_info_req(ids: Range<u32>) -> Vec<Envelope> {
    ids.map(build_user_info_req).collect()
}
