use clap::{App, Arg};
use dotenv::dotenv;
use std::env;
use ansi_term::Colour::{Green, White};

/// App Config
pub struct AppConfig {
    /// how many jobs to process
    pub jobs: usize,
    /// which path to parse
    pub path: String,
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
            .short("j")
            .long("jobs")
            .value_name("JOBS")
            .help("How many jobs (threads) in parallel should be ran.")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("PATH")
            .help("Which path to parse")
            .required(true)
            .index(1))
        .get_matches();
    let n_jobs = matches.value_of("JOBS").unwrap_or("1").parse::<usize>().unwrap();
    //    if let Some(ref n_jobs) = matches.value_of("JOBS").unwrap_or("1") {
    //        println!("number of jobs: {}", n_jobs);
    //    }
    let search_path = matches.value_of("PATH").unwrap();
    info!("processing path `{}` with {} threads",
          Green.paint(search_path.to_string()),
          Green.paint(n_jobs.to_string())
    );
    AppConfig { jobs: n_jobs, path: search_path.to_string() }
}