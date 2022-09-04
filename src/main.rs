#![allow(dead_code)]
use http::Method;
use crate::server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod server;
mod http;
mod website_handler;
fn main() {

    let default = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let path = env::var("PUBLIC_PATH").unwrap_or(default);

    println!("the public path is: {}", path);

    let string = String::from("127.0.0.1:8080");
    let server = Server::new(string);
    let _get = Method::GET;
    //let request = request::Request;
    server.run(WebsiteHandler::new(path));

}