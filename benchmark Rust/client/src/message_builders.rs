use protobuf::Message;

use super::NUMBER_OF_CELLS;

use rand::{thread_rng, Rng};

pub(crate) fn build_add_user(seq: u32) -> Vec<u8> {
    let mut rng = thread_rng();

    let mut msg = generated::ap::Ap::new();
    let req = msg.mut_add_user_req();
    req.seq_iq = seq;
    req.cell_id = seq % NUMBER_OF_CELLS;
    req.data = rng.gen();

    let mut buf: Vec<u8> = Vec::with_capacity(100);
    buf.push(msg.compute_size() as u8);
    let _ = msg.write_to_writer(&mut buf);

    buf
}

pub(crate) fn build_modify_user(user_id: u32) -> Vec<u8> {
    let mut rng = thread_rng();

    let mut msg = generated::ap::Ap::new();
    let req = msg.mut_modify_user_req();
    req.user_id = user_id;
    req.data = rng.gen();

    let mut buf: Vec<u8> = Vec::with_capacity(100);
    buf.push(msg.compute_size() as u8);
    let _ = msg.write_to_writer(&mut buf);

    buf
}

pub(crate) fn build_release_user(user_id: u32) -> Vec<u8> {
    let mut msg = generated::ap::Ap::new();
    let req = msg.mut_release_user_req();
    req.user_id = user_id;

    let mut buf: Vec<u8> = Vec::with_capacity(100);
    buf.push(msg.compute_size() as u8);
    let _ = msg.write_to_writer(&mut buf);

    buf
}

pub(crate) fn build_bulk_release(cell_id: u32) -> Vec<u8> {
    let mut msg = generated::ap::Ap::new();
    let req = msg.mut_bulk_release_req();
    req.cell_id = cell_id;

    let mut buf: Vec<u8> = Vec::with_capacity(100);
    buf.push(msg.compute_size() as u8);
    let _ = msg.write_to_writer(&mut buf);

    buf
}
