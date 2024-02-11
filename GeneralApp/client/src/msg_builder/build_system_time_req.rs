use generated::communication::*;

pub fn build_system_time_req() -> Envelope {
    let mut msg = Envelope::new();
    let _ = msg.mut_system_time_req();

    msg
}
