use generated::communication::*;
use std::ops::Range;

pub fn build_user_info_req(user_id: u32) -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_user_info_request();

    req.user_id = user_id;

    msg
}

pub fn generate_messages_user_info_req(ids: Range<u32>) -> Vec<Envelope> {
    ids.map(build_user_info_req).collect()
}
