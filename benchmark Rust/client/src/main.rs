use generated::ap::Ap;
use protobuf::Message;
use std::io::{BufReader, Read, Result, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Instant;

const NUMBER_OF_CELLS: u32 = 16;

mod message_builders;

fn read_message(reader: &mut BufReader<&TcpStream>) -> Result<Vec<u8>> {
    let mut size_buf = [0u8; 1];
    reader.read_exact(&mut size_buf)?;

    let payload_size = size_buf[0] as usize;
    let mut payload = vec![0u8; payload_size];
    reader.read_exact(&mut payload)?;

    Ok(payload)
}

fn handle_incoming_messages(stream: TcpStream, msg_count: usize) {
    let mut stream = BufReader::new(&stream);
    let mut cnt = msg_count;
    loop {
        match read_message(&mut stream) {
            Ok(payload) => match Ap::parse_from_bytes(&payload) {
                Ok(_) => {
                    cnt -= 1;
                    if cnt == 0 {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error deserializing message: {e}");
                }
            },
            Err(e) => {
                eprintln!("Error reading message: {}", e);
                break;
            }
        }
    }
}

fn generate_test_data() -> Vec<Vec<u8>> {
    let mut messages = Vec::new();
    let mut seq = 0;
    const USERS_COUNT: u32 = 20000;

    // Add users up up to limit
    for _ in 0..USERS_COUNT {
        messages.push(message_builders::build_add_user(seq));
        seq += 1;
    }

    // Add, remove, modify
    for _ in 0..USERS_COUNT {
        messages.push(message_builders::build_modify_user(seq % USERS_COUNT)); // initially added user
        messages.push(message_builders::build_add_user(seq));
        messages.push(message_builders::build_release_user(seq % USERS_COUNT)); // initially added user
        messages.push(message_builders::build_modify_user(seq));
        seq += 1;
    }

    // Bulk release with filling user storage to initial level
    for i in 0..NUMBER_OF_CELLS {
        messages.push(message_builders::build_bulk_release(i));
        for _ in 0..USERS_COUNT / NUMBER_OF_CELLS {
            messages.push(message_builders::build_add_user(seq));
            messages.push(message_builders::build_modify_user(seq));
            seq += 1;
        }
    }

    // Cleanup service
    for i in 0..NUMBER_OF_CELLS {
        messages.push(message_builders::build_bulk_release(i));
    }

    messages
}

fn main() {
    let data = generate_test_data();
    let size = data.iter().map(|inner_vec| inner_vec.len()).sum::<usize>();
    let count = data.len();
    println!("Generated {size} bytes of benchmark data in {count} messages");

    let stream = TcpStream::connect("127.0.0.1:6969").expect("Could not connect to server");
    let mut writer = stream.try_clone().expect("Failed to clone stream");

    let listener_handle = thread::spawn(move || {
        handle_incoming_messages(stream, count);
    });

    let start_time = Instant::now();
    for msg in &data {
        let _ = writer.write(msg);
    }

    // Wait for the receiver thread to finish
    listener_handle.join().expect("Listener thread panicked");
    let end_time = Instant::now();

    // Calculate the elapsed time
    let elapsed_time = end_time.duration_since(start_time);

    println!("Elapsed time {:?} ms", elapsed_time.as_millis());
    println!(
        "Average tput {:.0} msg/second",
        count as f64 / elapsed_time.as_millis() as f64 * 1000.0
    );
}
