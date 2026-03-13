use core::advance::ir::parse_to_ir;
use std::fs;

use crate::to_c::irs_to_c;

mod to_c;

fn main() {
    let code = fs::read_to_string("./box/bf/mandel.bf").unwrap();
    let ir = parse_to_ir(&code).unwrap();
    println!("{}", irs_to_c(&ir));
}
