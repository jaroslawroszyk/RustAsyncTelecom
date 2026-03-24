use envelope_macro::envelope_builder;
use generated::communication::{Envelope, Result as ProtoResult};

#[must_use]
#[envelope_builder(mut_user_info_response)]
pub fn build_user_info_response(username: String, result: ProtoResult) -> Envelope {
    inner.username = username;
    inner.result = result.into();
}
