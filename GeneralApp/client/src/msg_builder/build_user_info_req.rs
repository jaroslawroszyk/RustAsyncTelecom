use generated::communication::*;

pub fn build_user_info_req(user_id: u32, username: &str) -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_user_info_request();

    req.user_id = user_id;
    req.username = username.to_string();

    msg
}
