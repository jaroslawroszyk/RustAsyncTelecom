use generated::communication::*;

pub fn build_user_info_response(foo_req: &UserInfoRequest, result: Result) -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_user_info_response();

    req.username = format!("OK RECEIVED for {}", foo_req.username);
    req.result = result.into();

    msg
}
