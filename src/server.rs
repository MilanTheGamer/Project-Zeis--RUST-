use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    address:String
}

impl Server {
    pub fn new(address:String) -> Self {
        Self{
            address
        }
    }

    pub fn run(self) {
        println!("Server is running at address : {}",self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream,_)) => {
                    let mut buffer = [0;1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                          println!("{}",String::from_utf8_lossy(&buffer));  
                        },
                        Err(e) => {
                            println!("Faild to read from connection : {}",e)
                        }
                    }
                },
                Err(e) => {
                    println!("Faild to established connection: {}",e);
                }

            }
            // let req = listener.accept();

            // if req.is_err(){
            //     continue;
            // }

            // let (socket,addr) = req.unwrap();

        }
    }
}