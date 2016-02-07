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
use std::process::{Command, exit};
use ansi_term::Colour::{Yellow, Red, };
use pcap::{Capture, Device};

fn main(){
	logging::setup_logging();
	let command:options::Command;

    match options::parse_commandline_options(&env::args().collect()) {
        Ok(c) => {command = c;}
        Err(f) => { panic!("panic: {:?}", f) }
    };

	if command.list == true {
		list_devices();
		exit(0);
	}
	capture();
}

fn list_devices() {
	let devices = pcap::Device::list().ok().expect("Failed to list devices");

	if devices.len() > 0 {
		println!("Found {} devices:", Yellow.paint(&devices.len().to_string()));
	    for device in devices {
	        println!("- {}", Yellow.paint(&device.name.to_string()));
	    }
	} else {
		println!("{}", Red.paint("No devices found."));
		exit(1);
	}
}

fn capture() {
	// uses any (mostly `lo0`) device. pretty useless.
	// http://stackoverflow.com/a/3746101
	let mut cap = Capture::from_device(Device::lookup().unwrap()) // open the "default" interface
              .unwrap() // assume the device exists and we are authorized to open it
              .open() // activate the handle
              .unwrap(); // assume activation worked

    while let Some(packet) = cap.next() {
        println!("received packet! {:?}", packet);
    }
}