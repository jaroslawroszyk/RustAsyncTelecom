pub enum State {
    Initializing,
    SendingHeartbeatReq,
    SendingAddUserReq,
    SendingDeleteUserRequest,
    SendingUserInfoRequest,
    SendSystemTimeReq,
    WaitForHeartBeatResponse,
    WaitForAddUserResponse,
    WaitForDeleteUserResponse,
    WaitForUserInfoResponse,
    WaitForSystemTimeResp,
    Exit,
}
