extern crate easyopc;

use easyopc::*;

fn main() {
    let mut opc = PixelControl::default();
    let mut pixels = vec![Pixel{r:0,g:0,b:0}; 512];
    let mut i = 0;
    let colors = [
        Pixel{r:255,g:0,b:0},
        Pixel{r:0,g:255,b:0},
        Pixel{r:0,g:0,b:255},
    ];
    draw_with_interval_ms(1000, ||{
        i = (i + 1) % colors.len();
        let color = colors[i];

        for i in 0..512 {
            pixels[i] = color;
        }
        opc.emit(&pixels).unwrap();
    });
}
