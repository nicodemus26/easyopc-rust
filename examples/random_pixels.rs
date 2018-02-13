extern crate easyopc;

use easyopc::*;
use std::time::Duration;

fn main() {
    let mut opc = Connection::default();
    loop {
        let mut pixels = vec![Pixel{r:0,g:0,b:0}; 512];
        for i in 0..512 {
            pixels[i] = opc.random_color();
        }

        opc.emit(&pixels).unwrap();

        std::thread::sleep(Duration::from_millis(1000));
    }
}
