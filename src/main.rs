#[macro_use]
extern crate log;
extern crate fern;
extern crate time;
extern crate getopts;
extern crate ansi_term;


use std::env;
mod logging;
mod options;
use std::process::Command;
use std::string::String;
use std::io;

fn main(){
	logging::setup_logging();
    match options::parse_commandline_options(&env::args().collect()) {
        Ok(_) => {}
        Err(f) => { panic!("panic: {}", f) }
    };

	cmd("ls");

	info!("Info message");
	let x: i64 = 5;
	info!("x is {}", x);
}

/// try a system command
/// see:
/// 	- http://stackoverflow.com/questions/26478009/running-an-external-process-in-rust
/// 	- https://doc.rust-lang.org/core/result/
///
/// about strings in rust: http://stackoverflow.com/a/24159933
fn cmd(cmd: &str) {
	info!("enter a value: ");
	let mut input = String::new();
	io::stdin().read_line(&mut input)
		.ok()
		.expect("failed to read line");
	info!("hello {}", input);

	let process = Command::new(cmd).arg("-la").output().ok().expect("Failed to execute");
	let stdout = String::from_utf8(process.stdout).ok().expect("Failed to read");

	info!("{}", stdout);

	let split: Vec<&str> = stdout.split('\n').collect();
	for i in split {
		info!("     {}", i);
	}
}