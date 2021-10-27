#![allow(dead_code)]

mod server;
mod http;

use server::Server;

fn main() {
    let srv = Server::new("127.0.0.1:8080".to_string());
    srv.run();
}