pub mod initialize_client_exception;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResponseError {
    #[error("AddUserRespException")]
    AddUserRespException,
    #[error("DeleteUserResponseError")]
    DeleteUserResponseError,
    #[error("HeartBeatException")]
    HeartBeatException,
    #[error("InitializeClientException")]
    InitalizeClientException,
    #[error("SystemTimeResponseError")]
    SystemTimeResponseError,
    #[error("UserInfoResponseError")]
    UserInfoResponseError,
    #[error("Custom")]
    Custom,
}
