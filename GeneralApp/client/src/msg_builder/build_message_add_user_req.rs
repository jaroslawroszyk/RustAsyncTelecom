use generated::communication::*;

fn build_message_add_user_req(user_id: u32, user_name: &str) -> Envelope {
    let mut msg = Envelope::new();
    let req: &mut AddUserReq = msg.mut_add_user_req();

    req.user_id = user_id;
    req.user_name = user_name.to_string();

    msg
}

pub fn generate_messages() -> Vec<Envelope> {
    let user_ids = 1..= 3;
    let user_names = vec![
        "Alice", "Bob", "Charlie", "David", "Eva", "Frank", "Grace", "Henry", "Ivy", "Jack",
    ];

    user_ids
        .zip(user_names.into_iter())
        .map(|(id, name)| build_message_add_user_req(id, name))
        .collect()
}
