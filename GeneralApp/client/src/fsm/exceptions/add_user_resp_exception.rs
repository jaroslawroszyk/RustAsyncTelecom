#[derive(Debug)]
pub struct AddUserRespException;

impl std::fmt::Display for AddUserRespException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AddUserRespException occurs")
    }
}

impl std::error::Error for AddUserRespException {}

impl From<async_zmq::Error> for AddUserRespException {
    fn from(_: async_zmq::Error) -> Self {
        AddUserRespException
    }
}
