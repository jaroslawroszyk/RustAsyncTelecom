use generated::communication::*;

pub fn build_foo_response(foo_req: &FooReq) -> OperationMessage {
    let mut msg = OperationMessage::new();
    let req = msg.mut_foo_resp();
    req.user_name = format!("OK RECEIVED for {}", foo_req.user_name);

    msg
}
