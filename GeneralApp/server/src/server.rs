use async_zmq::{zmq, Context, Result};
use tokio::{net::TcpListener, time::Duration};

const SERVER_PORT: &str = "5556";

#[allow(dead_code)]
pub struct Server {
    context: Context,
    socket_address: String,
}

impl Server {
    pub async fn new() -> Result<Self> {
        if is_port_available(SERVER_PORT).await {
            let context = Context::new();
            let socket_address = format!("tcp://127.0.0.1:{}", SERVER_PORT);

            Ok(Server {
                context,
                socket_address,
            })
        } else {
            Err(async_zmq::Error::EADDRINUSE)
        }
    }

    pub async fn run(&self) -> Result<()> {
        let sub_socket = self.context.socket(zmq::SUB)?;
        sub_socket.bind(&self.socket_address)?;
        sub_socket.set_subscribe(b"")?;

        println!("Server is running and waiting for messages...");

        loop {
            let message: Vec<u8> = read_message(&sub_socket).await?;
            let (user_id, user_name) = extract_user_id_and_name(&message);
            println!("Received: User ID: {}, User Name: {}", user_id, user_name);

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

async fn read_message(socket: &zmq::Socket) -> Result<Vec<u8>> {
    let message = socket.recv_msg(0)?;
    print!("jarek read_message {:?}", socket.recv_msg(0)?);
    Ok(message.to_vec())
}

//tmp solution :)
fn extract_user_id_and_name(message: &[u8]) -> (u32, String) {
    if message.len() >= 8 {
        let user_id_bytes = &message[0..4];
        let user_id = u32::from_le_bytes(user_id_bytes.try_into().unwrap_or_default());

        if message.len() > 8 {
            let user_name_bytes = &message[7..];
            let user_name = String::from_utf8_lossy(user_name_bytes).to_string();

            return (user_id, user_name);
        }
    }

    (0, String::new())
}

async fn is_port_available(port: &str) -> bool {
    TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .is_ok()
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            socket_address: self.socket_address.clone(),
        }
    }
}

pub async fn run_server() -> Result<()> {
    let server = Server::new().await?;
    server.run().await?;
    Ok(())
}
