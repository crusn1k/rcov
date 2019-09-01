use clap::{clap_app, AppSettings};
use std::{str, process::{Command, exit}};
use regex::Regex;

fn main() {
    let (min_cov_pct, cov_tool, cov_tool_args) = get_cov_settings();
    
    let output = invoke_command(cov_tool, cov_tool_args);

    exit(check_coverage(output, min_cov_pct));
}

fn get_cov_settings() -> (f32, String, String) {
    let m =  clap_app!(myapp =>
        (version: "1.0")
        (author: "Nishikant Ajay Patil <crusnik02.trinityblood@gmail.com>")
        (about: "Wrapper over tarpaulin code coverage tool.")
        (setting: AppSettings::AllowExternalSubcommands)
        (@arg min_cov_pct: -m --mincovpct +takes_value "Minimum code coverate percentage.")
        (@arg quiet: -q --quiet "Quite mode for tests.")
    ).get_matches();

    let min_cov_pct: f32 = m.value_of("min_cov_pct").unwrap_or("80").parse().unwrap();
    let cov_tool: String;
    let mut cov_tool_args = String::from("");

    match m.subcommand() {
        (_cov_tool, Some(ext_m)) => {
            let _cov_tool_args: Vec<&str> = ext_m.values_of("").unwrap().collect();
            cov_tool_args = _cov_tool_args.join(" ").to_string();
            let _cov_tool = format!("cargo {}", _cov_tool);
            cov_tool = _cov_tool.to_string();
        },
        _ => {
            cov_tool = "cargo tarpaulin".to_string();
        },
    }

    println!(r"Coverage settings:
                Minimum line coverage required : {}%
                Coverage tool used : {}
                Coverage tool args : {}", min_cov_pct, cov_tool, cov_tool_args);
    
    (min_cov_pct, cov_tool, cov_tool_args)
}

fn invoke_command(cov_tool: String, cov_tool_args: String) -> String {

    let err_msg = format!("failed to run {}.", cov_tool);

    let cov_tool_cmd = format!("{} {}", cov_tool, cov_tool_args);

    let command = Command::new("sh").arg("-c").arg(cov_tool_cmd).output().expect(err_msg.as_str());

    let output = str::from_utf8(&command.stdout).unwrap();

    assert!(command.status.success());

    println!("{}", output);

    String::from(output)
}

fn check_coverage(output: String, min_cov_pct: f32) -> i32 {
    let rgx = Regex::new(r"([0-9]+(\.[0-9]+)?)% coverage").unwrap();
    
    let capture = match rgx.captures(output.as_str()) {
        Some(c) => c,
        None => return -2,
    };

    let coverage: f32 = capture.get(1).map_or("0", |c| c.as_str()).parse().unwrap(); 
    
    if coverage < min_cov_pct {
        println!("Test case coverage of {} is less than the minimum required coverage {}. Cannot proceed with this build.", coverage, min_cov_pct);
        return -1;
    }

    0
}

#[cfg(test)]
mod test_main {
   #[test]
    fn test_invoke_command() {
        let _output = crate::invoke_command("echo".to_string(), "81% coverage".to_string());
    }

    #[test]
    fn test_check_coverage() {
        assert_eq!(0, crate::check_coverage("81% coverage".to_string(), 81.0));
        assert_eq!(-1, crate::check_coverage("80% coverage".to_string(), 81.0));
        assert_eq!(-2, crate::check_coverage("foobar".to_string(), 81.0));
    }

    #[test]
    fn test_get_cov_settings() {
        crate::get_cov_settings();
    }
}