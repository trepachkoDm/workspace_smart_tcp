use std::net::ToSocketAddrs;
use stp::client::{RequestResult, StpClient};
use stp::error::ConnectResult;

pub struct SocketClient {
    stp: StpClient,
}

impl SocketClient {
    pub fn new<Addr: ToSocketAddrs>(addr: Addr) -> ConnectResult<Self> {
        let stp = StpClient::connect(addr)?;
        Ok(Self { stp })
    }

    pub fn switch_on(&mut self) -> RequestResult {
        let request = "switch_on|||".to_string();
        self.stp.send_request(request)
    }

    pub fn switch_off(&mut self) -> RequestResult {
        let request = "switch_off|||".to_string();
        self.stp.send_request(request)
    }

    pub fn get_state(&mut self) -> RequestResult {
        let request = "get_state|||".to_string();
        self.stp.send_request(request)
    }

    pub fn get_report(&mut self) -> RequestResult {
        let request = "get_report|||".to_string();
        self.stp.send_request(request)
    }
}
