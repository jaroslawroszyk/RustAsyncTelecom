#[derive(Debug)]
pub struct DeleteUserResponseError;

impl std::fmt::Display for DeleteUserResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to handle delete user response")
    }
}

impl std::error::Error for DeleteUserResponseError {}

impl From<async_zmq::Error> for DeleteUserResponseError {
    fn from(_: async_zmq::Error) -> Self {
        DeleteUserResponseError
    }
}
