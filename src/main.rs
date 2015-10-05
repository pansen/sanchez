#[macro_use]
extern crate log;
extern crate fern;
extern crate time;
extern crate pcap;
extern crate getopts;
extern crate ansi_term;


use std::env;
mod logging;
mod options;
use std::process::Command;

fn main(){
	logging::setup_logging();
	let command:options::Command;

    match options::parse_commandline_options(&env::args().collect()) {
        Ok(c) => {command = c;}
        Err(f) => { panic!("panic: {}", f) }
    };

	if command.list == true {
		list_devices();
	}

	// cmd("ls");
	info!("Info message");
	let x: i64 = 5;
	info!("x is {}", x);
}

fn list_devices() {
    for device in pcap::Device::list().unwrap() {
        println!("Found device! {:?}", device);
    }
}