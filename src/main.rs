use std::{env, str, process::Command};
use regex::Regex;

fn main() {
    let output = invoke_command("cargo tarpaulin");

    check_coverage(output.as_str());
}

fn invoke_command(command: &str) -> String {
    let args = get_args();

    let command = format!("{} {}", command, args);

    let err_msg = format!("failed to run {}.", command);

    let command = Command::new("sh").arg("-c").arg(command).output().expect(err_msg.as_str());

    let output = str::from_utf8(&command.stdout).unwrap();

    assert!(command.status.success());

    String::from(output)
}

fn check_coverage(output: &str) {
    let rgx = Regex::new(r"([0-9]+(\.[0-9]+)?)% coverage").unwrap();
    
    let capture = match rgx.captures(output) {
        Some(c) => c,
        None => return,
    };

    let coverage: f32 = capture.get(1).map_or("0", |c| c.as_str()).parse().unwrap(); 
    
    ::std::process::exit(match is_enough_test_coverage(coverage) {
       Ok(_) => 0,
       Err(err) => {
           eprintln!("error: {:?}", err);
           -1
       }
    });
}

fn is_enough_test_coverage(coverage : f32) -> Result<(), ()> {
    if coverage >= 80.0 {
        Ok(())
    } else {
        Err(())
    }
}

fn get_args() -> String {
    let args: Vec<String> = env::args().collect();

    let mut _args = String::new();

    for i in 1..args.len() {
        _args = format!("{} {} ", _args, args[i]);
    }
    _args
}