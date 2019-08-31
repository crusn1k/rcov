use clap::{Arg, App};
use std::{str, process::Command};
use regex::Regex;

fn main() {
    let args: String = get_arguments();

    let _style = format!("cargo {}", args);
    println!("Style {}",_style );
    let output = invoke_command(_style.as_str());

    check_coverage(output.as_str());
}

fn get_arguments() -> String {
        let matches = App::new("rcov")
                          .arg(Arg::with_name("covstyle")
                               .short("cs")
                               .long("covstyle")
                               .help("Runs test coverage style")
                               .takes_value(true))
                          .get_matches();
    let _style = matches.value_of("covstyle").unwrap_or("default.conf");

    let mut _args = String::new();
    _args = format!("{} {} ", _args, _style);
    println!("Format {}", _args);
    _args
}

fn invoke_command(command: &str) -> String {

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

#[cfg(test)]
mod test_main {
   #[test]
    fn test_invoke_command() {
        let output = crate::invoke_command("echo 81% coverage");

        crate::check_coverage(output.as_str());
    }
}