use generated::communication::*;

pub fn build_add_user_response(add_user_req: &AddUserReq) -> OperationMessage {
    let mut msg = OperationMessage::new();
    let req = msg.mut_add_user_resp();
    req.user_id = add_user_req.user_id;
    req.user_name = format!("OK RECEIVED for {}", add_user_req.user_name);

    msg
}
