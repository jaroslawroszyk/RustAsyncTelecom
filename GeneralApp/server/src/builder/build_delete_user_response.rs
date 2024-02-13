use generated::communication::*;

pub fn build_delete_user_response(delete_user_req: &DeleteUserRequest) -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_delete_user_response();

    // req.user_id =
    // req.username =

    msg
}
