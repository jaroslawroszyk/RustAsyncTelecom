use envelope_macro::envelope_builder;
use generated::communication::DeleteUserRequest;
use generated::communication::Envelope;
use generated::communication::Result;

#[envelope_builder(mut_delete_user_response)]
pub fn build_delete_user_response(
    delete_user_req: &DeleteUserRequest,
    delete_user_name: &str,
    result: Result,
) -> Envelope {
    inner.user_id = delete_user_req.user_id;
    inner.username = delete_user_name.to_string();
    inner.result = result.into();
}
