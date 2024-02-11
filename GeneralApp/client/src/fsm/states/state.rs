pub enum State {
    Initializing,
    SendingHeartbeatReq,
    SendingAddUserReq,
    SendingUserInfoRequest,
    SendSystemTimeReq,
    WaitForHeartBeatResponse,
    WaitForAddUserResponse,
    WaitForUserInfoResponse,
    WaitForSystemTimeResp,
    Exit,
}
