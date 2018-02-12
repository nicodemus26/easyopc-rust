extern crate easyopc;
extern crate rand;

use rand::Rng;
use std::io::Write;
use std::time::Duration;

fn main() {
    let mut ctx = easyopc::Connection::default();
    loop {
        let mut message = vec![0u8; 4 + (512 * 3)];
        // Command and channel both 0.
        message[2] = ((512u16 * 3) >> 8) as u8; // Length high byte
        message[3] = ((512u16 * 3) & 255) as u8; // Length low byte

        for i in 0..(512 * 3) {
            message[i + 4] = ctx.rng.gen();
        }

        ctx.stream.write_all(&message).unwrap();

        std::thread::sleep(Duration::from_millis(1000));
    }
}
