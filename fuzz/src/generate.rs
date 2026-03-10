use std::ops::Range;

use rand::{RngExt, rngs::ThreadRng};

pub fn generate_random_program(rng: &mut ThreadRng, size_range: &Range<usize>) -> String {
    let mut code = String::new();
    let mut level: i32 = 0;

    loop {
        if code.len() >= size_range.end {
            code.push_str(&"]".repeat(level as usize));
            return code;
        }
        match rng.random_range(0u8..7) {
            0 => {
                code.push('+');
            }
            1 => {
                code.push('-');
            }
            2 => {
                code.push('<');
            }
            3 => {
                code.push('>');
            }
            4 => {
                code.push('[');
                level += 1;
            }
            5 => {
                if level == 0 {
                    if size_range.start <= code.len() {
                        return code;
                    } else {
                        continue;
                    }
                }
                level -= 1;
                code.push(']')
            }
            6 => {
                code.push('.');
            }
            _ => unreachable!(),
        }
    }
}
