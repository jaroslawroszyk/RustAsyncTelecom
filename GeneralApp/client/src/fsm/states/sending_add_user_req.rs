use anyhow::Result;
use async_zmq::zmq;

use anyhow::anyhow;
use generated::communication::*;

use crate::fsm::send_message;

pub async fn sending_add_user_req(
    socket: &zmq::Socket,
    iter: &mut impl Iterator<Item = &Envelope>,
) -> Result<()> {
    if let Some(message) = iter.next() {
        send_message(socket, message).await?;

        Ok(())
    } else {
        Err(anyhow!("No more messages to send"))
    }
}
