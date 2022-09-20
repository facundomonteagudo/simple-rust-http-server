// by default everything created in a new file is treated like a module.
use crate::http::Request;
use std::io::Read;
use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {
    // Associated fn
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    // Method allways have a reference to self
    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recived a request: {}", String::from_utf8_lossy(&buffer));

                            Request::try_from(&buffer[..]);
                        }
                        Err(e) => println!("READ ERROR: {}", e),
                    }
                }
                Err(e) => print!("ERROR: {}", e),
            }
        }
    }
}
