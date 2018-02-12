extern crate easyopc;
extern crate rand;
extern crate rgb;

use rand::Rng;
use rgb::{FromSlice, RGB8};
use std::time::Duration;

fn main() {
    let mut ctx = easyopc::Connection::default();
    loop {
        let mut message = vec![0u8; 512*3];
        for i in 0..(512 * 3) {
            message[i] = ctx.rng.gen();
        }

        ctx.emit(&message.as_rgb()).unwrap();

        std::thread::sleep(Duration::from_millis(1000));
    }
}
