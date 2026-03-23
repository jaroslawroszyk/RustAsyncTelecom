use generated::communication::{AddUserReq, Envelope};
use std::ops::Range;

fn build_message_add_user_req(user_id: u32, user_name: &str) -> Envelope {
    let mut msg = Envelope::new();
    let req: &mut AddUserReq = msg.mut_add_user_req();

    req.user_id = user_id;
    req.user_name = user_name.to_string();

    msg
}

#[must_use]
pub fn generate_messages_add_user_req(ids: Range<u32>) -> Vec<Envelope> {
    let user_names = vec![
        "Alice", "Bob", "Charlie", "David", "Eva", "Frank", "Grace", "Henry", "Ivy", "Jack",
    ];

    ids.zip(user_names.into_iter().cycle())
        .map(|(id, name)| build_message_add_user_req(id, name))
        .collect()
}
