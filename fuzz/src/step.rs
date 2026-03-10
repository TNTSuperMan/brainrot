use std::fs;

use rand::{RngExt, rngs::ThreadRng};

use crate::{Config, emu::{EmuErr, exec_emu}, exec::{ExecResult, execute}, generate::generate_random_program};

fn report(rng: &mut ThreadRng, code: &str, message: &str) {
    fs::write(&format!("./box/fuzz/{}.bf", rng.random::<u32>()), format!("[\n{message}\n]\n{code}")).unwrap();
}

pub fn step(rng: &mut ThreadRng, config: &Config) {
    let code = generate_random_program(rng, &config.size_range);

    let emures = exec_emu(&code, config.timeout_step);
    if let Err(EmuErr::Timeout) = emures {
        return;
    }

    let execres = execute(&config.path, &code, config.timeout_ms);
    match execres {
        ExecResult::Ok(exout) => {
            if let Ok(emout) = emures {
                if exout == emout {
                    print!(".");
                } else {
                    report(rng, &code, "output unmatched");
                    println!("!");
                }
            } else {
                report(rng, &code, "oob expected but success");
                println!("!");
            }
        },
        ExecResult::Timeout => {
            // report(rng, &code, "timeout");
            println!("_");
        },
        ExecResult::Err(_e) => {
            if let Err(_) = emures {
                println!("_");
            } else {
                report(rng, &code, "success expected but oob");
                println!("!");
            }
        },
        ExecResult::Panic(panic) => {
            println!("{panic}");
            report(rng, &code, &format!("panic: {panic}"));
        },
        ExecResult::Core(core) => {
            println!("{core}");
            report(rng, &code, &format!("CORE DUMPED: {core}"));
        },
    }
}
