use std::fmt;

#[derive(Debug, Clone)]
pub struct HeartBeatException {}

impl fmt::Display for HeartBeatException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HeartBeatException occurs")
    }
}
