use anyhow::Result;
use generated::communication::Envelope;
use protobuf::Message;

/// Serializes a protobuf message into a byte vector.
/// # Arguments
/// * `msg` - A reference to the protobuf message to be serialized.
/// # Returns
/// A byte vector containing the serialized message.
/// # Panics
/// This function will panic if the message cannot be serialized, which should not happen under normal circumstances
/// # Errors
/// This function will return an error if the message cannot be serialized, which should not happen under normal circumstances
pub fn serialize_message(msg: &Envelope) -> Result<Vec<u8>> {
    let size = usize::try_from(msg.compute_size())?;
    let mut buf = Vec::with_capacity(size);
    msg.write_to_vec(&mut buf)?;
    Ok(buf)
}
