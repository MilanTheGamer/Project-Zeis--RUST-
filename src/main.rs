fn main() {
    let server = Server::new("ZEUS".to_string());
    server.run();
}

struct Server {
    server_name:String
}

impl Server {
    fn new(server_name:String) -> Self {
        Self{
            server_name
        }
    }

    fn run(self) {
        println!("{} is running",self.server_name)
    }
}


