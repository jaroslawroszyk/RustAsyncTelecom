use generated::communication::AddUserReq;
use generated::communication::Envelope;
use generated::communication::Result;
use envelope_macro::envelope_builder;

#[envelope_builder(mut_add_user_resp)]
pub fn build_add_user_response(add_user_req: &AddUserReq, result: Result) -> Envelope {
    inner.user_id = add_user_req.user_id;
    inner.user_name = format!("OK RECEIVED for {}", add_user_req.user_name);
    inner.result = result.into();
}
