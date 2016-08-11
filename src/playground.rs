#[macro_use] extern crate log;
extern crate fern;
extern crate time;
extern crate pcap;
extern crate getopts;
extern crate ansi_term;


use std::env;

mod logging;
mod options;

use std::process::Command;
use std::string::String;
use std::io;
use pcap::Device;

fn main() {
    logging::setup_logging();
    cmd("ls");
    info!("Info message");
    let x: i64 = 5;
    info!("x is {}", x);
}

#[allow(dead_code)]
fn simple_capture(device: pcap::Device) {
    // now you can create a Capture with this Device if you want.
    let mut cap = device.open().unwrap();

    // get a packet from this capture
    let packet = cap.next();

    println!("got a packet! {:?}", packet);
}


/// try a system command
/// see:
/// 	- http://stackoverflow.com/questions/26478009/running-an-external-process-in-rust
/// 	- https://doc.rust-lang.org/core/result/
///
/// about strings in rust: http://stackoverflow.com/a/24159933
#[allow(dead_code)]
fn cmd(cmd: &str) {
    info!("enter a value: ");
    let process = Command::new(cmd).arg("-la").output().ok().expect("Failed to execute");
    let stdout = String::from_utf8(process.stdout).ok().expect("Failed to read");

    info!("{}", stdout);

    let split: Vec<&str> = stdout.split('\n').collect();
    for i in split {
        info!("     {}", i);
    }
}

#[allow(dead_code)]
fn read_input() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .ok()
        .expect("failed to read line");
    info!("hello {}", input);
}