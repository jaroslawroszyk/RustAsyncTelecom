use anyhow::Result;
use async_zmq::zmq::{self};
use dotenv_codegen::dotenv;
use rand::Rng;

pub async fn initialize_client(socket: &zmq::Socket) -> Result<()> {
    let mut rng = rand::thread_rng();
    let client_id: String = rng.gen_range(1000..9999).to_string();

    socket.set_identity(client_id.as_bytes())?;
    match socket.connect(&dotenv!("IP_ADDRESS")) {
        Err(e) => eprintln!("No connection to the server. Cannot send messages. ERR: {e}"),
        Ok(_) => {
            println!("Connected to the server at {:?}", dotenv!("IP_ADDRESS"));
        }
    };

    Ok(())
}
