extern crate rand;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};

use std::env;
use image::{ImageBuffer, Pixel, Rgb};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!{"{:?}",args};

    let (width, height) = (1920, 1080);
    let mut rng = rand::thread_rng();
    let mut img = ImageBuffer::new(width,height);
    for x in 0..width {
        for y in 0..height {
            let (r,g,b): (u8,u8,u8) = rng.gen();
            let pxl = Rgb::from_channels(r,g,b,0);
            img.put_pixel(x, y, pxl);
        }
    }
    let _ = img.save("output.png");
}
