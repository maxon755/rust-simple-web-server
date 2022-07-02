use super::Request;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buf = [0; 1024];

                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request {}", String::from_utf8_lossy(&buf));

                            let request = Request::try_from(&buf as &[u8]).unwrap();

                            dbg!(request);
                        }
                        Err(e) => println!("Failed to read from connection {}", e),
                    }
                }

                Err(e) => println!("Failed to establish a connction: {}", e),
            }
        }
    }
}
