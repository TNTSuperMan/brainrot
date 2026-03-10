use std::{io::{Read, Write}, os::unix::process::ExitStatusExt, process::{Command, Stdio}, time::Duration};

use wait_timeout::ChildExt;

#[derive(Debug)]
pub enum ExecResult {
    Ok(Vec<u8>),
    Timeout,
    Err(String),
    Panic(String),
    Core(String),
}

pub fn execute(path: &str, code: &str, timeout_ms: u64) -> ExecResult {
    println!("ex");
    let duration = Duration::from_millis(timeout_ms);

    let mut child = Command::new(path)
        .arg("/dev/stdin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn().unwrap();
    
    child.stdin.as_mut().unwrap().write_all(code.as_bytes()).unwrap();

    match child.wait_timeout(duration).unwrap() {
        Some(status) => {
            let stderr_res: Result<Vec<u8>, _> = child.stderr.unwrap().bytes().collect();
            let stderr = String::from_utf8(stderr_res.unwrap()).unwrap();

            if status.core_dumped() {
                return ExecResult::Core(stderr);
            }
            if status.success() {
                let stdout_res: Result<Vec<u8>, _> = child.stdout.unwrap().bytes().collect();
                return ExecResult::Ok(stdout_res.unwrap());
            }
            if status.code().unwrap() == 101 {
                return ExecResult::Panic(stderr);
            }
            return ExecResult::Err(stderr);
        }
        None => {
            println!("timeout");
            child.kill().unwrap();
            println!("killed");
            child.wait().unwrap();
            println!("waited");
            return ExecResult::Timeout;
        }
    }
}
