use generated::communication::{Envelope, Result as ProtoResult}; // Zmiana nazwy przy imporcie

#[must_use]
pub fn build_user_info_response(username: String, result: ProtoResult) -> Envelope {
    let mut msg = Envelope::new();
    let resp = msg.mut_user_info_response();

    resp.username = username;
    resp.result = result.into();

    msg
}
