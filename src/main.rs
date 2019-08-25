extern crate rand;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};

use std::env;
use image::{ImageBuffer, Pixel, Rgb};

// if anyone asks you didn't see this
pub fn random() -> (u8, u8, u8) {
    unsafe {
        static mut STATE: u64 = 0x123456789abcdef0;
        STATE = STATE.wrapping_mul(2862933555777941757)
            .wrapping_add(3037000493);
        (((STATE >> 56) & 0xff) as u8,
        ((STATE >> 48) & 0xff) as u8,
        ((STATE >> 40) & 0xff) as u8)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (width, height) = if args.len() == 3 {
        (match args[1].parse::<u32>() {
            Ok(num) => num,
            Err(_) => 1920,
        },
        match args[2].parse::<u32>() {
            Ok(num) => num,
            Err(_) => 1080,
        })
    }
    else {
        (1920, 1080)
    };

    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(1..255);
    let mut img = ImageBuffer::new(width,height);
    for x in 0..width {
        for y in 0..height {
            let (r,g,b): (u8,u8,u8) = rng.gen::<(u8, u8, u8)>();
            let pxl = Rgb::from_channels(r,g,b,0);
            img.put_pixel(x, y, pxl);
        }
    }
    let _ = img.save("output.png");
}
