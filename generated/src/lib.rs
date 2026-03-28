pub mod communication;

use protobuf::Message;

pub trait ProtoSerialize: Message {
    /// Serializes the protobuf message into bytes.
    /// # Errors
    /// Returns `protobuf::Error` if serialization fails.
    fn serialize(&self) -> Result<Vec<u8>, protobuf::Error> {
        self.write_to_bytes()
    }
}

impl ProtoSerialize for communication::Envelope {}
