pub mod initialize_client_exception;

use std::fmt;

#[derive(Debug)]
pub enum ResponseError {
    AddUserRespException,
    DeleteUserResponseError,
    HeartBeatException,
    InitalizeClientException,
    SystemTimeResponseError,
    UserInfoResponseError,
    Custom,
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseError::AddUserRespException => write!(f, "AddUserRespException"),
            ResponseError::DeleteUserResponseError => write!(f, "DeleteUserResponseError"),
            ResponseError::HeartBeatException => write!(f, "HeartBeatException"),
            ResponseError::InitalizeClientException => write!(f, "InitalizeClientException"),
            ResponseError::SystemTimeResponseError => write!(f, "SystemTimeResponseError"),
            ResponseError::UserInfoResponseError => write!(f, "UserInfoResponseError"),
            ResponseError::Custom => write!(f, "Custom"),
        }
    }
}
