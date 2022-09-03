
pub mod server {
    use std::{net::TcpListener, io::Read};

    use crate::http::Request;

    pub struct Server {
        addr: String
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Self {
                addr
            }
        }
    
        pub fn run(self){
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

                                match Request::try_from(&buffer[..]) {
                                    Ok(_request) => {
                                        
                                    }
                                    Err(e) =>{
                                        print!("Filed to parse a request: {}", e);
                                    }
                                }


                                println!("Received: {}", String::from_utf8_lossy(&buffer));
                            },
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
}