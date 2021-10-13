use std::io::Read;
use std::net::TcpListener;
use std::convert::TryFrom;
use crate::http::Request;

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
                          match Request::try_from(&buffer[..]) {
                              Ok(request) => {
                                unimplemented!();
                                // dbg!(request);
                              },
                              Err(e) => {
                                println!("{}",e)
                              }
                          } 
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