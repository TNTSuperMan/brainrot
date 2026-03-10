use std::io::{Write, stdout};

use rand::rng;

use crate::{exec::{ExecResult, execute}, generate::generate_random_program};

mod generate;
mod exec;
mod emu;

fn main() {
    let mut rng = rng();
    let mut stdout = stdout().lock();
    for _ in 0..1000 {
        let code = generate_random_program(&mut rng, &(100..1000), 10);
        let res = execute("../target/debug/cli", &code, 100);
        match res {
            ExecResult::Ok(_out) => print!("."),
            ExecResult::Timeout => print!("_"),
            ExecResult::Err(_e) => print!("!"),
            ExecResult::Panic(panic) => println!("\nPanic!: {panic}"),
            ExecResult::Core(core) => println!("\nCORE DUMPED!: {core}"),
        }
        stdout.flush().unwrap();
    }
}
