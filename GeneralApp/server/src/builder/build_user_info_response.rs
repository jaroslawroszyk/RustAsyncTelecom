use generated::communication::*;

pub fn build_user_info_response(
    foo_req: &UserInfoRequest,
    username: String,
    result: Result,
) -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_user_info_response();

    req.username = username;
    req.result = result.into();

    msg
}
