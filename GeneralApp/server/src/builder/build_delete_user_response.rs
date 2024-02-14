use generated::communication::Result;
use generated::communication::*;

pub fn build_delete_user_response(
    delete_user_req: &DeleteUserRequest,
    delete_user_name: &str,
    result: Result,
) -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_delete_user_response();

    req.user_id = delete_user_req.user_id;
    req.username = delete_user_name.to_string();
    req.result = result.into();

    msg
}
