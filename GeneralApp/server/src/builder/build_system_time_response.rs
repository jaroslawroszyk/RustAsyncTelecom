use chrono::Utc;
use generated::communication::*;

pub fn build_system_time_response() -> Envelope {
    let mut msg = Envelope::new();
    let resp = msg.mut_system_time_resp();
    let current_time = Utc::now();
    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S UTC").to_string();

    resp.current_time = formatted_time;

    msg
}
