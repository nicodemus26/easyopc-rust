extern crate rand;

use rand::Rng;
use std::env;
use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};
use std::time::Duration;

fn main() {
    let endpoint = env::var("OPC_ENDPOINT")
        .unwrap_or(String::from("127.0.0.1:7890"));
    let mut stream = TcpStream::connect(endpoint).unwrap();
    stream.shutdown(Shutdown::Read).unwrap(); // Not a great listener...
    stream.set_nodelay(true).unwrap();
    let mut rng = rand::thread_rng();

    loop {
        let mut message = vec![0u8; 4+(512*3)];
        // Command and channel both 0.
        message[2] = ((512u16*3) >> 8)  as u8; // Length high byte
        message[3] = ((512u16*3) & 255) as u8; // Length low byte
        
        for i in 0..(512*3) {
            message[i+4] = rng.gen();
        }
        
        stream.write_all(&message).unwrap();
        
        std::thread::sleep(Duration::from_millis(1000));
    }
}
