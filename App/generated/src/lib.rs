pub mod communication;

use protobuf::Message;

pub trait ProtoSerialize: Message {
    fn serialize(&self) -> Vec<u8> {
        self.write_to_bytes()
            .expect("Failed to serialize protobuf message")
    }
}

impl ProtoSerialize for communication::Envelope {}
