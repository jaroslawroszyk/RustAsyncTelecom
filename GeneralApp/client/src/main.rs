use async_zmq::{zmq, Context, Result};
use tokio::time::Duration;

async fn run_client() -> Result<()> {
    let context = Context::new();
    let socket = context.socket(zmq::SUB)?;
    socket.connect("tcp://127.0.0.1:5555")?;
    socket.set_subscribe(b"")?;

    loop {
        if let Ok(message) = socket.recv_msg(0) {
            let message_str = message.as_str().unwrap_or_default();
            println!("Received: {}", message_str);
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    run_client().await?;
    Ok(())
}
