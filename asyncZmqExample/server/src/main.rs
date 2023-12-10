use async_zmq::{Context, Result, zmq};
use tokio::time::Duration;

async fn run_server() -> Result<()> {
    let context = Context::new();

    let socket = context.socket(zmq::PUB)?;
    socket.bind("tcp://127.0.0.1:5555")?;

    loop {
        let message = "Hello, world!";
        socket.send(message, 0)?;

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    run_server().await?;
    Ok(())
}
