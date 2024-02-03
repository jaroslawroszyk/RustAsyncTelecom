pub enum State {
    Initializing,
    SendingHeartbeatReq,
    SendingAddUserReq,
    SendingUserInfoRequest,
    WaitForHeartBeatResponse,
    WaitForAddUserResponse,
    WaitForUserInfoResponse,
    Exit,
}
