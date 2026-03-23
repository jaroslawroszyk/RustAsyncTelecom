use anyhow::Result;
use async_zmq::zmq::{self};
use dotenv_codegen::dotenv;
use rand::Rng;

use crate::fsm::exceptions::initialize_client_exception::InitalizeClientException;

pub async fn initialize_client(socket: &zmq::Socket) -> Result<(), InitalizeClientException> {
    let mut rng = rand::thread_rng();
    let client_id: String = rng.gen_range(1000..9999).to_string();

    socket.set_identity(client_id.as_bytes()).map_err(|_| {
        InitalizeClientException::ConnectionFailed("Failed to set client identity".to_string())
    })?;

    if socket.connect(dotenv!("IP_ADDRESS")).is_err() {
        return Err(InitalizeClientException::ConnectionFailed(format!(
            "No connection to the server at {:?}",
            dotenv!("IP_ADDRESS")
        )));
    } else {
        log::info!("Connected to the server at {:?}", dotenv!("IP_ADDRESS"));
    }

    Ok(())
}
