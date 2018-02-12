extern crate rand;

use std::net::{Shutdown, TcpStream};
use std::env;
use std::default::Default;

pub struct Connection {
    pub rng: rand::ThreadRng,
    pub stream: TcpStream,
}

impl Default for Connection {
    fn default() -> Connection {
        let endpoint = env::var("OPC_ENDPOINT").unwrap_or(String::from("127.0.0.1:7890"));
        let stream = TcpStream::connect(endpoint).unwrap();
        stream.shutdown(Shutdown::Read).unwrap(); // Not a great listener...
        stream.set_nodelay(true).unwrap();
        let rng = rand::thread_rng();
        Connection {
            rng: rng,
            stream: stream,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
