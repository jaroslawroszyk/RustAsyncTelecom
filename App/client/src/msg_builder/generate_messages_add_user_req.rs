use envelope_macro::envelope_builder;
use generated::communication::Envelope;
use std::ops::Range;

#[envelope_builder(mut_add_user_req)]
fn build_message_add_user_req(user_id: u32, user_name: &str) -> Envelope {
    inner.user_id = user_id;
    inner.user_name = user_name.to_string();
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
