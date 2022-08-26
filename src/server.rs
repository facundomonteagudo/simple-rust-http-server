// by default everything created in a new file is treated like a module.
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
            listener.accept();
        }
    }
}
