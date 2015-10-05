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
use ansi_term::Colour::{Yellow, };

fn main(){
	logging::setup_logging();
	let command:options::Command;

    match options::parse_commandline_options(&env::args().collect()) {
        Ok(c) => {command = c;}
        Err(f) => { panic!("panic: {:?}", f) }
    };

	if command.list == true {
		list_devices();
	}
}

fn list_devices() {
	let devices = pcap::Device::list().ok().expect("Failed to list devices");

	if devices.len() > 0 {
		println!("Found {} devices:", Yellow.paint(&devices.len().to_string()));
	    for device in devices {
	        println!("- {}", Yellow.paint(&device.name.to_string()));
	    }
	}
}