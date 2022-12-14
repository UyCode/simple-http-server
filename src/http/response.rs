use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};
use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Response {
        Response { status_code, body }
    }

    // method 1 with generic type
    pub fn send_tcp(&self, stream: &mut impl Write) -> IoResult<()> {unimplemented!()}
    // method 2 with concrete type
    pub fn send_file(&self, stream: &mut File) -> IoResult<()> {unimplemented!()}

    // method 3 with concrete type
    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

// for demo purposes
impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.status_code)
        //unimplemented!()
    }
}