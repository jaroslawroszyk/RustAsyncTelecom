use async_zmq::{zmq, Context, Result};
use generated::company::*;
use protobuf::Message;
use tokio::time::Duration;

const PORT: &str = "5556";

fn build_message(user_id: u32) -> SomeMsg {
    let mut msg = SomeMsg::new();
    let req = msg.mut_add_user();

    req.user_id = user_id;
    req.user_name = "panicName".into();

    msg
}

fn serialize_message(msg: &SomeMsg) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    msg.write_to_vec(&mut buf)
        .expect("Failed to serialize message");
    buf
}

fn generate_messages() -> Vec<SomeMsg> {
    let user_ids = 69..80;
    user_ids.map(build_message).collect()
}

pub async fn run_client() -> Result<()> {
    let context = Context::new();
    let socket = context.socket(zmq::PUB)?;
    let address = format!("tcp://127.0.0.1:{PORT}");

    match socket.connect(&address) {
        Err(e) => eprintln!("No connection to the server. Cannot send messages. ERR: {e}"),
        Ok(_) => println!("Connected to the server at tcp://127.0.0.1:{PORT}"),
    };

    // Without delay, the first message will be sent before the connection is established and the message will be lost
    tokio::time::sleep(Duration::from_millis(1)).await;

    for message in &generate_messages() {
        println!("Sent message: {message}");
        let serialized_msg = serialize_message(message);
        socket.send(&serialized_msg, 0)?;
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    Ok(())
}
