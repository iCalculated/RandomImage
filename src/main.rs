extern crate rand;
// _ is the wildcard symbol, like a * import in Python
// when I used an "as _" it failed to find the function:
// error[E0425]: cannot find function `thread_rng` in this scope
use rand::Rng as _;
use rand::thread_rng;
use std::env;
use image::{ImageBuffer, Rgb};

fn extract_arg_int(arg: Option<&String>, default: u32) -> u32 {
    // and_then is a flat map to prevent nested Options
    // I don't understand how a closure differs from a lambda
    // but for each value it sets arg to the flattened (right term?)
    // result.
    // unwrap_or is nicer than match for checking Option status
    arg.and_then(|arg| arg.parse().ok())
        .unwrap_or(default)
}

fn main() {
    // _ implies type, so definition can be more succint
    let args: Vec<_> = env::args().collect();

    // No longer a matching mess
    let width = extract_arg_int(args.get(1), 1920);
    let height = extract_arg_int(args.get(2), 1080);

    let mut img = ImageBuffer::new(width,height);
    let mut rng = thread_rng();

    // entirely forgot rust has mutable iteration
    for pixel in img.pixels_mut() {
        *pixel = Rgb(rng.gen());
    }

    // unwrap the result instead of sending it to the void
    // if there is an error it will panic
    img.save("output.png").unwrap();
}

