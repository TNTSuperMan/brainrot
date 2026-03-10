use rand::rng;

use crate::generate::generate_random_program;

mod generate;

fn main() {
    let mut rng = rng();
    for _ in 0..10000 {
        let code = generate_random_program(&mut rng, &(100..10000));
    }
}
