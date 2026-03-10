use std::{ops::Range, time::Instant};

use rand::rng;

use crate::step::step;

mod generate;
mod exec;
mod emu;
mod step;

pub struct Config {
    path: String,
    timeout_step: u64,
    timeout_ms: u64,
    size_range: Range<usize>,
}

fn main() {
    let mut rng = rng();
    let config = Config {
        path: "target/debug/brainrot".to_string(),
        timeout_step: 1000,
        timeout_ms: 100,
        size_range: 50..200,
    };
    loop {
        let start = Instant::now();
        for _ in 0..1000 {
            step(&mut rng, &config);
        }
        let end = Instant::now();
        println!("{:?}", (end - start) / 1000);
    }
}
