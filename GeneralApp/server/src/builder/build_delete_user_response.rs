use generated::communication::Result;
use generated::communication::*;

pub fn build_delete_user_response(delete_user_req: &DeleteUserRequest, result: Result) -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_delete_user_response();

    req.user_id = delete_user_req.user_id;
    req.username = delete_user_req.username.clone();
    req.result = result.into();

    msg
}
