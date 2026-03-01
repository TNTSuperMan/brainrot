use crate::{bytecode::bytecode::Bytecode, error::BrainrotError, vm::{program::Program, tape::Tape, tier::{internal::Tier, run}}};

mod program;
mod tape;
mod tier;

pub fn run_cisc<I: FnMut() -> u8, O: FnMut(u8) -> ()>(insts: Box<[Bytecode]>, timeout: Option<usize>, input: I, output: O) -> Result<(), BrainrotError> {
    let mut tape = Tape::new();
    let mut program = Program::new(insts, timeout, input, output);
    let mut tier = Tier::Deopt;
    if cfg!(feature = "trace") {
        println!("[TRACE] first: {:?}", tier);
    }

    run(&mut tier, &mut tape, &mut program)
}
