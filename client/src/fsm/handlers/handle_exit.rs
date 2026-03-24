use anyhow::Result;
use async_zmq::zmq;
use dotenv_codegen::dotenv;

/// Handles the exit process for the client application.
/// It logs the exit message and attempts to disconnect the `ZeroMQ` socket from the server
/// using the IP address specified in the environment variable `IP_ADDRESS`.
/// # Errors
/// This function will return an error if it fails to disconnect the socket, which could happen due to network issues, an invalid socket state, or if the specified IP address is incorrect or unreachable.
pub async fn handle_exit(socket: &zmq::Socket) -> Result<()> {
    logger::info!("Exiting..");
    if let Err(e) = socket.disconnect(dotenv!("IP_ADDRESS")) {
        logger::error!("Error disconnecting socket: {:?}", e);
    }
    Ok(())
}
