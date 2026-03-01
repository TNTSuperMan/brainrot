use crate::{bytecode::bytecode::Bytecode, error::BrainrotError, vm::{program::Program, tape::Tape, tier::{internal::Tier, run}}};

mod program;
mod tape;
mod tier;

pub fn run_cisc(insts: &[Bytecode], timeout: Option<usize>) -> Result<(), BrainrotError> {
    let mut tape = Tape::new();
    let mut program = Program::new(insts, timeout);
    let mut tier = Tier::Deopt;
    if cfg!(feature = "trace") {
        println!("[TRACE] first: {:?}", tier);
    }

    run(&mut tier, &mut tape, &mut program)
}
