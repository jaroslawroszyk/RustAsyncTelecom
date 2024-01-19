use generated::communication::*;
use protobuf::Message;

pub fn serialize_message(msg: &OperationMessage) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    msg.write_to_vec(&mut buf)
        .expect("Failed to serialize message");
    buf
}
