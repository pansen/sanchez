#[macro_use]
extern crate log;
extern crate fern;
extern crate time;
extern crate clap;
extern crate ansi_term;

mod logging;
use std::process::{exit, };
use ansi_term::Colour::{Yellow, };
use clap::{App, Arg};

fn main(){
	logging::setup_logging();

    let matches = App::new("CSV Importer")
		.version("0.0.1")
		.author("pansen")
    	.arg(Arg::with_name("CSV_PATH")
			.help("Sets the path where to look for csv files")
			.required(true)
			.index(1))
    	.get_matches();

	let csv_path = matches.value_of("CSV_PATH").unwrap();

	debug!("csv path is {}", Yellow.paint(csv_path));
	exit(0);
}
