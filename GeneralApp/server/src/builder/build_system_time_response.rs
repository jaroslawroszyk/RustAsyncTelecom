use chrono::Utc;
use generated::communication::Result;
use generated::communication::*;

pub fn build_system_time_response(result: Result) -> Envelope {
    let mut msg = Envelope::new();
    let req = msg.mut_system_time_resp();
    let current_time = Utc::now();
    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S UTC").to_string();

    req.current_time = formatted_time;
    req.result = result.into();

    msg
}
