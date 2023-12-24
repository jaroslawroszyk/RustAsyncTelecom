use async_zmq::{zmq, Context, Result};
use std::net::TcpListener;
use tokio::time::Duration;

const SERVER_PORT: &str = "5555";

#[allow(dead_code)]
pub struct Server {
    context: Context,
    socket_address: String,
}

impl Server {
    pub async fn new() -> Result<Self> {
        if is_port_available(SERVER_PORT) {
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
        let socket = self.context.socket(zmq::PUB)?;

        socket.bind(&self.socket_address)?;

        loop {
            let message = "Hello, world!";
            socket.send(message, 0)?;

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

//For tests purposes
impl Clone for Server {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            socket_address: self.socket_address.clone(),
        }
    }
}

fn is_port_available(port: &str) -> bool {
    TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok()
}

pub async fn run_server() -> Result<()> {
    let server = Server::new().await?;
    server.run().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use async_zmq::Message;

    use super::*;

    #[tokio::test]
    async fn test_run_server() {
        let server = Server::new().await.unwrap();

        let server_clone = server.clone();
        let server_handle = tokio::spawn(async move {
            server_clone.run().await.unwrap();
        });

        tokio::time::sleep(Duration::from_secs(1)).await;

        let context = Context::new();
        let subscriber = context.socket(zmq::SUB).unwrap();
        subscriber
            .connect(&format!("tcp://127.0.0.1:{}", SERVER_PORT))
            .unwrap();
        subscriber.set_subscribe(b"").unwrap();

        tokio::time::sleep(Duration::from_secs(2)).await;

        let received_message = subscriber.recv_msg(0).unwrap();
        let expected_message = Message::from("Hello, world!");
        assert_eq!(received_message, expected_message);

        server_handle.abort();
    }

    #[tokio::test]
    async fn test_server_address_not_available() {
        let _context = Context::new();
        let _listener = TcpListener::bind(format!("127.0.0.1:{}", SERVER_PORT)).unwrap();
        let result = Server::new().await;

        match result {
            Err(async_zmq::Error::EADDRINUSE) => {
                dbg!("Expected EADDRINUSE, the port is in use");
                assert!(true);
            }
            Ok(_) => {
                dbg!("Unexpected success, the port should be in use");
                assert!(false, "Expected EADDRINUSE, but got success");
            }
            Err(err) => {
                dbg!("Unexpected error: {:?}", err);
                assert!(false, "Unexpected error: {:?}", err);
            }
        }
    }
}
