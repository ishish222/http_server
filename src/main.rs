#![allow(dead_code)]

mod server;
mod http;
mod website_handler;

use server::Server;
use http::Request;
use http::Method;
use website_handler::WebsiteHandler;
use std::env;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("The public path is equal to {}", public_path);
    let server = Server::new("0.0.0.0:8080".to_string());
    server.run(WebsiteHandler::new(public_path));

}
