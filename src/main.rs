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

use std::process::{exit, };

fn main() {
    let config = arguments::parse();

    logging::setup_logging(&config);

    scan::scanner(&config);

    exit(0);
}

