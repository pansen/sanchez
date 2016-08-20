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
extern crate crypto;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

mod logging;
mod path;
mod arguments;
mod scan;
mod watch;

use std::process::{exit, };
use std::thread;
use std::vec::Vec;
use std::env;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;


fn main() {
    let config = arguments::parse();

    let r2d2_config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").unwrap());
    let pool = r2d2::Pool::new(r2d2_config, manager).expect("Failed to create pool.");

    logging::setup_logging(&config);

    let mut watcher_handles: Vec<thread::JoinHandle<_>> = Vec::with_capacity(1);

    if config.watch == true {
        let config = config.clone();
        // TODO amb: would be nicer to share only *one* `Scanner` here, but i don't know how exactly
        let scanner_thread = scan::Scanner::new(&config);

        watcher_handles.push(thread::spawn(move || {
            let _ = watch::watch_reference(&config, &scanner_thread);
        }));
    }

    let scanner = scan::Scanner::new(&config);
    scanner.scan_all();

    // after the scanner is done, we just join the watcher thread to avoid the mainthread to exit
    // creating a channel could also be done, but seems like overhead here
    // http://stackoverflow.com/a/26200583/2741111
    for handle in watcher_handles {
        let _ = handle.join();
    }
    exit(0);
}

