#[macro_use]
extern crate criterion;
extern crate rand;

use criterion::Criterion;

use rand::Rng;
use rand::distributions::{Distribution, Uniform};

fn get_uniform() -> (u8, u8, u8) {
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(1..255);
    (uniform.sample(&mut rng), uniform.sample(&mut rng),  uniform.sample(&mut rng))
}

fn get_thread() -> (u8, u8, u8) {
    let mut rng = rand::thread_rng();
    rng.gen::<(u8, u8, u8)>()
}

pub fn random() -> u8 {
    unsafe {
        static mut STATE: u64 = 0x123456789abcdef0;
        STATE = STATE.wrapping_mul(2862933555777941757)
            .wrapping_add(3037000493);
        (STATE % 256) as u8
    }
}

pub fn tri_random() -> (u8, u8, u8) {
    (random(), random(),random())
}
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("uniform random", |b| b.iter(|| get_uniform()));
    c.bench_function("thread random", |b| b.iter(|| get_thread()));
    c.bench_function("bad random", |b| b.iter(|| tri_random()));
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
