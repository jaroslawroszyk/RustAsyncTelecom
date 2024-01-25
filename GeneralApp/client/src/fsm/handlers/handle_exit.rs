use anyhow::Result;
use async_zmq::zmq;
use dotenv_codegen::dotenv;

pub async fn handle_exit(socket: &zmq::Socket) -> Result<()> {
    if let Err(e) = socket.disconnect(&dotenv!("IP_ADDRESS")) {
        eprintln!("Error disconnecting socket: {:?}", e);
    }
    Ok(())
}
