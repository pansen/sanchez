#[macro_use]
extern crate log;
extern crate fern;
extern crate time;
extern crate clap;
extern crate ansi_term;

extern crate csv;
extern crate dotenv;
extern crate rustc_serialize;
extern crate threadpool;


mod logging;
mod csv_import;

use std::process::{exit, };
use ansi_term::Colour::{Yellow, Green, Red, White};
use clap::{App, Arg};
use dotenv::dotenv;
use std::env;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;


fn main(){
	logging::setup_logging();
	dotenv().ok();

    for (key, value) in env::vars().filter(|tuple| tuple.0 == "FOO") {
    	println!("dotenv: {}: {}", Green.paint(key), White.paint(value));
    }


	let n_workers = 4;
	let n_jobs = 8;
	let pool = ThreadPool::new(n_workers);

	let (tx, rx) = channel();
	for job in 0..n_jobs {
	    let tx = tx.clone();
	    pool.execute(move|| {
	    	debug!("sending {} from thread", Yellow.paint(job.to_string()));
	        tx.send(job.to_string()).unwrap();
	    });
	}

	for value in rx.iter().take(n_jobs) {
		debug!("receiving {} from thread", Green.paint(value));
	}
	exit(0);

}

