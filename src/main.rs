#[macro_use]
extern crate log;
extern crate fern;
extern crate time;

mod logging;
use std::process::Command;

fn main(){
	logging::setup_logging();

	cmd("ls");

	info!("Info message");
	let x: i64 = 5;
	info!("x is {}", x);
}

/// try a system command
fn cmd(cmd: &str) {
	let output = Command::new(cmd).arg("-la").output().unwrap_or_else(|e| {
	  panic!("failed to execute process: {}", e)
	});
	info!("output: {}", output.stdout[0]);
}