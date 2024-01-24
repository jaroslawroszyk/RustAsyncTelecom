use generated::communication::*;

pub fn build_foo_req() -> OperationMessage {
    let mut msg = OperationMessage::new();
    let req = msg.mut_foo_req();

    req.user_name = "JAREK JESTEM".into();

    msg
}
