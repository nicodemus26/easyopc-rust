# easyopc - Open Pixel Control in Rust
An ergonomics-first [Open Pixel Control](http://openpixelcontrol.org/) client for devices like [Fadecandy](http://www.misc.name/fadecandy/) and [Total Control Lighting](http://www.coolneon.com/category/total-control-lighting/).

## Usage
```rust
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
```

## Stability
This library is currently pre-1.0, which means breaking changes will happen. Consider pinning to a specific version to not have your build break.

My intention for this library is to create a small set of examples and simplify the api as needed to make the examples easier, more succinctly expressive, and simpler. Once I have a diverse set of examples at a satisfactory local minimum of complexity, I'll stabilize the library. Please open an issue if you have an example that's overly difficult to implement.

## Configuration
Configuration is drawn from environment variables with sensible defaults.


## Examples
### Random Pixels

Sets ever pixel in the canvas to a random RGB value every second.

```cargo run --example random_pixels```

### RGB Rotate

Rotates through red, green, and blue every 10 seconds. This demonstrates crossfading well.

```cargo run --example rgb_rotate```
