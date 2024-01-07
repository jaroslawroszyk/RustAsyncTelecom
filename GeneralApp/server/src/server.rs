use async_zmq::{zmq, Context, Result};
use generated::company::*;
use protobuf::Message;
use tokio::net::TcpListener;

const PORT: &str = "5556";

#[derive(Clone)]
pub struct Server {
    context: Context,
    socket_address: String,
}

impl Server {
    pub async fn new() -> Result<Self> {
        if !is_port_available(PORT).await {
            return Err(async_zmq::Error::EADDRINUSE);
        }

        Ok(Server {
            context: Context::new(),
            socket_address: format!("tcp://127.0.0.1:{PORT}"),
        })
    }

    pub async fn run(&self) -> Result<()> {
        let socket = self.context.socket(zmq::SUB)?;
        socket.bind(&self.socket_address)?;
        socket.set_subscribe(b"")?;

        println!("Server is running and waiting for messages...");

        loop {
            let message: Vec<u8> = read_message(&socket).await?;

            match SomeMsg::parse_from_bytes(&message) {
                Ok(msg) => match msg.msgtype {
                    Some(some_msg::Msgtype::AddUser(ref msg)) => {
                        println!("Received message: add_user {{{msg}}}");
                    }
                    _ => eprintln!("Received unsupported message: {msg}"),
                },
                Err(e) => eprintln!("Unable to deserialize message: {e}"),
            }
        }
    }
}

async fn read_message(socket: &zmq::Socket) -> Result<Vec<u8>> {
    let message = socket.recv_msg(0)?;
    Ok(message.to_vec())
}

async fn is_port_available(port: &str) -> bool {
    TcpListener::bind(format!("127.0.0.1:{port}")).await.is_ok()
}

pub async fn run_server() -> Result<()> {
    let server = Server::new().await?;
    server.run().await?;
    Ok(())
}
