#[derive(Debug)]
pub struct SystemTimeResponseError;

impl std::fmt::Display for SystemTimeResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to handle system time response")
    }
}
