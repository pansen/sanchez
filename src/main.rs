#[macro_use] extern crate log;
extern crate fern;

extern crate time;
extern crate clap;
extern crate ansi_term;

extern crate csv;
extern crate dotenv;
extern crate rustc_serialize;
extern crate threadpool;
extern crate walkdir;
extern crate id3;
extern crate notify;


mod logging;
mod path;
mod arguments;
mod scan;
mod watch;

use std::process::{exit, };

fn main() {
    let config = arguments::parse();

    logging::setup_logging(&config);

    let scanner = scan::Scanner::new(&config);

    if config.watch == true {
        watch::watch_reference(&config);
    }

    scanner.scan_all();

    exit(0);
}

