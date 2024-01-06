use async_zmq::{zmq, Context, Result};
use generated::company::*;
use protobuf::Message;
use tokio::time::{timeout, Duration};

fn build_msg(user_id: u32) -> SomeMsg {
    let mut msg = SomeMsg::new();
    let req = msg.mut_add_user();
    req.user_id = user_id;
    req.user_name = "panicName".to_string();

    print!("jarek user_id {}", req.user_id);

    msg
}

fn serialize_msg(msg: &SomeMsg) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let _ = msg
        .write_to_vec(&mut buf)
        .expect("Failed to serialize message");
    buf
}

fn generate_data() -> Vec<SomeMsg> {
    let mut messages = Vec::new();
    let mut user_id = 69;

    for _ in 0..10 {
        messages.push(build_msg(user_id));
        user_id += 1;
    }

    messages
}

pub async fn run_client() -> Result<()> {
    let context = Context::new();
    let socket = context.socket(zmq::PUB)?;
    socket.connect("tcp://127.0.0.1:5556")?;

    let data = generate_data();
    println!("Client is running and should send");

    for message in &data {
        let serialized_msg = serialize_msg(message);
        println!("jarek serialized_msg {:?}", serialized_msg);

        socket.send(&serialized_msg, 0)?;

        tokio::time::sleep(Duration::from_millis(2)).await;
    }

    Ok(())
}
