#![allow(dead_code)]
use http::Method;

use server::server::Server;
mod server;
mod http;

fn main() {

    let string = String::from("127.0.0.1:8080");
    let server = Server::new(string);
    let _get = Method::GET;
    //let request = request::Request;
    server.run();

}