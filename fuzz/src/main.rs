use rand::rng;

use crate::{exec::execute, generate::generate_random_program};

mod generate;
mod exec;

fn main() {
    let mut rng = rng();
    for _ in 0..1000 {
        let code = generate_random_program(&mut rng, &(100..1000), 10);
        println!("{:?}", execute("../target/debug/cli", &code, 1000));
    }
}
