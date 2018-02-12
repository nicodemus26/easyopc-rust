extern crate easyopc;
extern crate rand;

use easyopc::*;
use rand::Rng;
use std::time::Duration;

fn main() {
    let mut opc = Connection::default();
    loop {
        let pixels = &mut [Pixel{r:0,g:0,b:0}; 512];
        for pixel in pixels.iter_mut() {
            pixel.r = opc.rng.gen();
            pixel.g = opc.rng.gen();
            pixel.b = opc.rng.gen();
        }

        opc.emit(pixels).unwrap();

        std::thread::sleep(Duration::from_millis(1000));
    }
}
