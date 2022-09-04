use std::{net::TcpListener, io::Read, convert::TryFrom};
use crate::http::{ParseError, Request, Response, StatusCode};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listing on {}", self.addr);


        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("Accepted connection");
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            //Request::try_from(&buffer[..]);
                            //let result: &Result<Request, _> = &buffer[..].try_into();

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    /*dbg!(request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>it works</h1>".to_string()))*/

                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                    //Response::new(StatusCode::BadRequest, None)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }


                            println!("Received: {}", String::from_utf8_lossy(&buffer));
                        }
                        Err(e) => {
                            println!("Error: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }

        }
    }
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        print!("Filed to parse a request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}