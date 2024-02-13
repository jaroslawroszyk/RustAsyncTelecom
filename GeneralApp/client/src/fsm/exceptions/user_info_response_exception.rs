#[derive(Debug)]
pub struct UserInfoResponseError;

impl std::fmt::Display for UserInfoResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to handle user info response")
    }
}

impl std::error::Error for UserInfoResponseError {}

impl From<async_zmq::Error> for UserInfoResponseError {
    fn from(_: async_zmq::Error) -> Self {
        UserInfoResponseError
    }
}
