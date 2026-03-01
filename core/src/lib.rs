const TAPE_LENGTH: usize = 65536;

pub mod error;
mod ir;
mod bytecode;
mod vm;
mod trace;

pub mod advance {
    pub use crate::ir::ir::parse_to_ir;
}
