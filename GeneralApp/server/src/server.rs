use async_zmq::{zmq, Context, Result};
use generated::company::*;
use protobuf::Message;
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

            if message.is_empty() {
                continue;
            }

            let serialized_msg = &message[1..];

            match generated::company::SomeMsg::parse_from_bytes(serialized_msg) {
                Ok(msg) => match msg.msgtype {
                    Some(some_msg::Msgtype::AddUser(ref msg)) => {
                        let user_id = msg.user_id;
                        println!("Deserialized: User ID: {}", user_id);
                    }
                    _ => {
                        panic!("Unsupported msg type");
                    }
                },
                Err(e) => {
                    eprintln!("Error deserializing message: {:?}", e);
                }
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

async fn read_message(socket: &zmq::Socket) -> Result<Vec<u8>> {
    let message = socket.recv_msg(0)?;
    print!("jarek read_message {:?}", socket.recv_msg(0)?);
    Ok(message.to_vec())
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
