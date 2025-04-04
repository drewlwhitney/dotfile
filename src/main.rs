// use std::fs;
use std::process::Command;

pub mod pac;
// use pac::*;

fn main() {
    Command::new("sudo")
        .args(["pacman", "-Rs", "solaar"])
        .status()
        .expect("Fuck");

    // let output = Command::new("ls")
    //     .current_dir(fs::canonicalize(".").expect("Fuck"))
    //     .output()
    //     .expect("Failed")
    //     .stdout;
    // let output = String::from_utf8(output).expect("Failed to unwrap");
    // let output: Vec<&str> = output.lines().collect();
    // for item in output.iter().by_ref() {
    //     println!("{}", item);
    // }
}
