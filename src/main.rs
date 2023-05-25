#![allow(dead_code)]

mod server;
mod http;

use server::Server;
use http::Request;
use http::Method;

fn main() {

    let server = Server::new("0.0.0.0:8080".to_string());
    server.run();

}
