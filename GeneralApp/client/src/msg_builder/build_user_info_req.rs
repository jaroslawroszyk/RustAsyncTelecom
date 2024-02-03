use generated::communication::*;

pub fn build_user_info_req() -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_user_info_request();

    req.username = "CLIENT".into();

    msg
}
