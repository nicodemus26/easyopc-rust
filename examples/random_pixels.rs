extern crate easyopc;

use easyopc::*;

fn main() {
    let mut opc = PixelControl::default();
    let mut pixels = vec![Pixel{r:0,g:0,b:0}; 512];
    draw_with_interval_ms(1000, ||{
        for i in 0..512 {
            pixels[i] = opc.random_color();
        }
        opc.emit(&pixels).unwrap();
    });
}
