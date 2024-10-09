use generated::communication::Result;
use generated::communication::*;

pub fn build_add_user_response(add_user_req: &AddUserReq, result: Result) -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_add_user_resp();

    req.user_id = add_user_req.user_id;
    req.user_name = format!("OK RECEIVED for {}", add_user_req.user_name);
    req.result = result.into();

    msg
}
