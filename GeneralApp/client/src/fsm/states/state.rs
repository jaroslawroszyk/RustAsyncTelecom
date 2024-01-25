pub enum State {
    Initializing,
    SendingHeartbeatReq,
    SendingAddUserReq,
    SendingFooReq,
    WaitForHeartBeatResponse,
    WaitForAddUserResponse,
    WaitForFooResponse,
    Exit,
}
