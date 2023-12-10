mod service;

use service::UeService;
use std::io::{BufReader, Read, Result, ErrorKind};
use std::net::{TcpListener, TcpStream};

fn read_message(reader: &mut BufReader<&TcpStream>) -> Result<Vec<u8>> {
    let mut size_buf = [0u8; 1];
    reader.read_exact(&mut size_buf)?;

    let payload_size = size_buf[0] as usize;
    let mut payload = vec![0u8; payload_size];

    reader.read_exact(&mut payload)?;

    Ok(payload)
}

fn handle_connection(stream: TcpStream) {
    //stream
    let writer = stream.try_clone().unwrap();
    let mut reader = BufReader::new(&stream);

    //ue service
    let mut ue_service = UeService::new();

    loop {
        match read_message(&mut reader) {
            Ok(message) => {
                ue_service.handle_message(&message, &writer);
            }
            Err(err) => match err.kind() {
                ErrorKind::UnexpectedEof => {
                    eprintln!("Connection closed by peer");
                    break;
                }
                _ => { continue; }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6969")?;
    println!("Service started. Waiting for incomming connections");

    for stream in listener.incoming() {
        handle_connection(stream?);
    }
    Ok(())
}
