use std::fs;

use rand::{RngExt, rngs::ThreadRng};

use crate::{Config, emu::exec_emu, exec::{ExecResult, execute}, generate::generate_random_program};

fn report(rng: &mut ThreadRng, code: &str, message: &str) {
    fs::write(&format!("./box/fuzz/{}.bf", rng.random::<u32>()), format!("[\n{message}\n]\n{code}")).unwrap();
}

pub fn step(rng: &mut ThreadRng, config: &Config) {
    let code = generate_random_program(rng, &config.size_range);

    let emures = exec_emu(&code, config.timeout_step);
    if let Ok(emout) = emures {
        let execres = execute(&config.path, &code, config.timeout_ms);
        match execres {
            ExecResult::Ok(exout) => {
                if exout == emout {
                    print!(".");
                } else {
                    report(rng, &code, "output unmatched");
                    print!("!");
                }
            },
            ExecResult::Timeout => {
                // report(rng, &code, "timeout");
                print!("_");
            },
            ExecResult::Err(e) => {
                report(rng, &code, &format!("unexpected err: {e}"));
                print!("!");
            },
            ExecResult::Panic(panic) => {
                println!("\n{panic}");
                report(rng, &code, &format!("panic: {panic}"));
            },
            ExecResult::Core(core) => {
                println!("\n{core}");
                report(rng, &code, &format!("CORE DUMPED: {core}"));
            },
        }
    }
}
