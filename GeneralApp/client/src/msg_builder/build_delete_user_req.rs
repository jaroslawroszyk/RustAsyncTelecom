use generated::communication::*;

pub fn build_delete_user_req() -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_delete_user_request();

    req.user_id = 1;
    req.username = "Alice".to_owned();

    msg
}
