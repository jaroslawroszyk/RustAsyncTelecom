use generated::communication::*;
use protobuf::Message;

pub fn serialize_message(msg: &Envelope) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    msg.write_to_vec(&mut buf)
        .expect("Failed to serialize message");
    buf
}
