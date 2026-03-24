use chrono::Local;
use envelope_macro::envelope_builder;
use generated::communication::Envelope;
use generated::communication::Result;

#[must_use]
#[envelope_builder(mut_system_time_resp)]
pub fn build_system_time_response(result: Result) -> Envelope {
    let current_time = Local::now();
    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S").to_string();

    inner.current_time = formatted_time;
    inner.result = result.into();
}
