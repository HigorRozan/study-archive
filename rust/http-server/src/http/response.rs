use std::io::{Result as IoResult, Write};
use std::net::TcpStream;

use super::StatusCode;

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }

    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        let empty_body = "".to_string();
        let body = self.body.as_ref().unwrap_or(&empty_body);
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
