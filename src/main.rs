#[macro_use]
extern crate log;
extern crate fern;
extern crate time;
extern crate clap;
extern crate ansi_term;

extern crate csv;
extern crate dotenv;
extern crate rustc_serialize;


mod logging;
mod csv_import;

use std::process::{exit, };
use ansi_term::Colour::{Yellow, Green, Red, White};
use clap::{App, Arg};
use dotenv::dotenv;
use std::env;


fn main(){
	logging::setup_logging();
	dotenv().ok();

    for (key, value) in env::vars().filter(|tuple| tuple.0 == "FOO") {
    	println!("dotenv: {}: {}", Green.paint(key), value);
    }

    let matches = App::new("CSV Importer")
		.version("0.0.1")
		.author("pansen")
    	.arg(Arg::with_name("CSV_PATH")
			.help("Sets the path where to look for csv files")
			.required(true)
			.index(1))
    	.get_matches();

	let csv_path = matches.value_of("CSV_PATH").unwrap();
	debug!("csv path is {}", Green.paint(csv_path));

	csv_import::parse_csv_file(csv_path);
	exit(0);
}

