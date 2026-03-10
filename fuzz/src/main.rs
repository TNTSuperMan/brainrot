use std::{io::{Write, stdout}, ops::Range};

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
    let mut stdout = stdout().lock();
    let config = Config {
        path: "target/debug/brainrot".to_string(),
        timeout_step: 1000,
        timeout_ms: 100,
        size_range: 100..500,
    };
    for _ in 0..100000 {
        step(&mut rng, &config);
        stdout.flush().unwrap();
    }
    println!("");
}
