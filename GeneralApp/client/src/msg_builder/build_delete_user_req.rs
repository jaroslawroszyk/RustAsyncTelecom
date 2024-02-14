use generated::communication::*;

pub fn build_delete_user_req(id: u32) -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_delete_user_request();

    req.user_id = id;

    msg
}
