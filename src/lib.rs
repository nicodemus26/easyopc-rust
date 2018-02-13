extern crate rand;
extern crate rgb;

use rand::Rng;
use rgb::*;
use std::default::Default;
use std::env;
use std::io::{Result, Write};
use std::net::{Shutdown, TcpStream};

pub struct PixelControl {
    pub rng: rand::ThreadRng,
    pub stream: TcpStream,
}

pub type Pixel = rgb::RGB8;

impl PixelControl {
    pub fn emit(&mut self, pixels: &[Pixel]) -> Result<()> {
        let mut header = vec![0u8; 4];
        // Command and channel both 0.
        header[2] = ((512u16 * 3) >> 8) as u8; // Length high byte
        header[3] = ((512u16 * 3) & 255) as u8; // Length low byte
        self.stream.write_all(&header)?;
        self.stream.write_all(pixels.as_bytes())
    }

    pub fn random_color(&mut self) -> Pixel {
        Pixel {
            r: self.rng.gen(),
            g: self.rng.gen(),
            b: self.rng.gen(),
        }
    }
}

impl Default for PixelControl {
    fn default() -> PixelControl {
        let endpoint = env::var("OPC_ENDPOINT").unwrap_or(String::from("127.0.0.1:7890"));
        let stream = TcpStream::connect(endpoint).unwrap();
        stream.shutdown(Shutdown::Read).unwrap(); // Not a great listener...
        stream.set_nodelay(true).unwrap();
        let rng = rand::thread_rng();
        PixelControl {
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
