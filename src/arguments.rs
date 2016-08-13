use clap::{App, Arg};
use dotenv::dotenv;
use std::env;
use ansi_term::Colour::{Green, White};

/// App Config
pub struct AppConfig {
    /// how many jobs to process
    pub jobs: usize,
    /// how many workers (threads) we shall use
    pub workers: usize,
}

/// parses the given arguments our app
pub fn parse() -> AppConfig {
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
    info!("processing {} jobs with {} threads", Green.paint(n_jobs.to_string()),
          Green.paint(n_workers.to_string()));
    AppConfig { jobs: n_jobs, workers: n_workers }
}