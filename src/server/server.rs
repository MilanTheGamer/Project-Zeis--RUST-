use std::io::{Read, Write};
use std::net::TcpListener;
use std::convert::TryFrom;
use crate::http::{Request, Response, StatusCode};

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
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>HELLO</h1>".to_string())
                                    )
                                },
                                Err(e) => {
                                    Response::new(
                                        StatusCode::NotFound,
                                        None
                                    )
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Faild to send response : {} ", e);
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
        }
    }
}