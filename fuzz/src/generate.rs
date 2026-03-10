use std::ops::Range;

use rand::{RngExt, rngs::ThreadRng};

pub fn generate_random_program(rng: &mut ThreadRng, size_range: &Range<usize>, loop_max: i32) -> String {
    let mut code = String::new();
    let mut level: i32 = 0;

    loop {
        if code.len() >= size_range.end {
            code.clear();
            level = 0;
        }
        match rng.random_range(0u8..8u8) {
            0 => {
                code.push_str("+");
            }
            1 => {
                code.push_str("-");
            }
            2 => {
                code.push_str("<");
            }
            3 => {
                code.push_str(">");
            }
            4 => {
                if loop_max > level {
                    code.push_str("[");
                    level += 1;
                }
            }
            5 => {
                level -= 1;
                if level == -1 {
                    if size_range.start > code.len() {
                        level = 0;
                        continue;
                    } else {
                        return code;
                    }
                }
                code.push_str("]");
            }
            6 => {
                code.push_str(".");
            }
            7 => {
                code.push_str(",");
            }
            _ => unreachable!(),
        }
    }
}
