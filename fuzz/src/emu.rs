use std::collections::HashMap;

pub enum EmuErr {
    Timeout,
    OOB,
}

pub fn exec_emu(code: &str, timeout: u64) -> Result<Vec<u8>, EmuErr> {
    let mut stdout: Vec<u8> = vec![];

    let ops: Vec<char> = code.chars().collect();

    let jumptable = {
        let mut jumptable = HashMap::<usize, usize>::new();
        let mut stack: Vec<usize> = vec![];
        for (i, c) in ops.iter().enumerate() {
            match c {
                '[' => stack.push(i),
                ']' => {
                    let addr = stack.pop().unwrap();
                    jumptable.insert(addr, i);
                    jumptable.insert(i, addr);
                }
                _ => {}
            }
        }
        jumptable
    };

    let mut memory = [0u8; 65536];
    let mut pointer = 0usize;
    let mut step = 0u64;
    let mut pc = 0usize;

    while timeout > step {
        if ops.len() <= pc {
            return Ok(stdout);
        }
        match ops[pc] {
            '+' => {
                let cell = memory.get_mut(pointer).ok_or(EmuErr::OOB)?;
                *cell = cell.wrapping_add(1);
            }
            '-' => {
                let cell = memory.get_mut(pointer).ok_or(EmuErr::OOB)?;
                *cell = cell.wrapping_sub(1);
            }
            '>' => pointer = pointer.wrapping_add(1),
            '<' => pointer = pointer.wrapping_sub(1),
            '[' => {
                if *memory.get(pointer).ok_or(EmuErr::OOB)? == 0 {
                    pc = *jumptable.get(&pc).unwrap();
                }
            }
            ']' => {
                if *memory.get(pointer).ok_or(EmuErr::OOB)? != 0 {
                    pc = *jumptable.get(&pc).unwrap();
                }
            }
            '.' => stdout.push(*memory.get(pointer).ok_or(EmuErr::OOB)?),
            _ => {

            }
        }
        step += 1;
        pc += 1;
    }

    Err(EmuErr::Timeout)
}
