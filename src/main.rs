mod server;

use server::Server;

fn main() {
    let srv = Server::new("127.0.0.1:8080".to_string());
    srv.run();
}