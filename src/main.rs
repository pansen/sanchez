#[macro_use] extern crate log;
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
use std::time::Duration;
use std::thread;

fn main() {
    logging::setup_logging();
    dotenv().ok();

    for (key, value) in env::vars().filter(|tuple| tuple.0 == "FOO") {
        println!("dotenv: {}: {}", Green.paint(key), White.paint(value));
    }

    let matches = App::new("Rust Playground")
        .version("0.0.1")
        .author("pansen")
        .arg(Arg::with_name("JOBS")
            .help("How many jobs will be executed")
            .required(true)
            .index(1))
        .arg(Arg::with_name("WORKERS")
            .help("How many threads will work the jobs")
            .required(true)
            .index(2))
        .get_matches();
    let n_jobs = matches.value_of("JOBS").unwrap().parse::<usize>().unwrap();
    let n_workers = matches.value_of("WORKERS").unwrap().parse::<usize>().unwrap();
    info!("processing {} jobs with {} threads", Yellow.paint(n_jobs.to_string()), Yellow.paint(n_workers.to_string()));

    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for job in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            debug!("sending {} from thread", Yellow.paint(job.to_string()));
            thread::sleep(Duration::from_millis(1000));
            tx.send(job.to_string()).unwrap();
        });
    }

    for value in rx.iter().take(n_jobs) {
        debug!("receiving {} from thread", Green.paint(value));
    }
    exit(0);
}

