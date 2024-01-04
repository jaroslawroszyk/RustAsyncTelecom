use async_zmq::{zmq, Context, Result};
use protobuf::Message;
use tokio::time::Duration;

fn build_msg(user_id: u32) -> Vec<u8> {
    let mut msg = generated::company::SomeMsg::new();
    let req = msg.mut_add_user();
    req.user_id = user_id;
    req.user_name = "panicName".to_string();

    // print!("jarek req {:?}", req);

    let mut buf: Vec<u8> = Vec::new();
    let _ = msg.write_to_vec(&mut buf);

    // println!("jarek msg {:?}", msg.write_to_vec(&mut buf));
    buf
}

fn generate_data() -> Vec<Vec<u8>> {
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
    println!("jarek generated data {:?}", data);

    for message in data {
        let size = message.len() as u8;
        print!("size {}", size);
        let mut full_message = vec![size];
        full_message.extend_from_slice(&message);

        socket.send(full_message, 0)?;

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
