
use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let status = Command::new("./target/release/tkexecute")
        .args(&args)
        .status()
        .expect("Error (In Program): failed to run tkexecute");

    if !status.success() {
        eprintln!("Error (In Tkexecute): tkexecute failed");
    }
}

