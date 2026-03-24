use anyhow::Result;
use async_zmq::zmq::{self};
use dotenv_codegen::dotenv;
use rand::Rng;

use crate::fsm::exceptions::initialize_client_exception::InitalizeClientException;

/// Initializes the client by setting a unique identity and connecting to the server.
/// # Arguments
/// * `socket` - A reference to the `ZeroMQ` socket used for communication with the server
/// # Returns
/// A Result indicating the success or failure of the initialization process.
/// # Errors
/// This function will return an error if the client fails to set its identity or connect to the server.
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
    }
    logger::info!("Connected to the server at {:?}", dotenv!("IP_ADDRESS"));

    Ok(())
}
