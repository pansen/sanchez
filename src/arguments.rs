use clap::{App, Arg};
use dotenv::dotenv;
use std::env;
use ansi_term::Colour::{Green, White};
use num_cpus;

/// App Config
/// this is marked as clonable (implements `std::clone::Clone`)
#[derive(Clone, Debug)]
pub struct AppConfig {
    /// how many jobs to process
    pub jobs: usize,
    /// which path to parse
    pub path: String,
    /// level of verbosity
    pub verbose: usize,
    /// enable directory watching
    pub watch: bool,
}

/// parses the given arguments our app
pub fn parse() -> AppConfig {
    dotenv().ok();

    for (key, value) in env::vars().filter(|tuple| tuple.0 == "DATABASE_URL") {
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
        .arg(Arg::with_name("VERBOSE")
            .short("v")
            .help("Verbosity level (repeat for even more verbosity).")
            .multiple(true)
            .required(false)
            .takes_value(false))
        .arg(Arg::with_name("WATCH")
            .short("w")
            .long("watch")
            .help("Enable watcher for changes in the given path.")
            .required(false)
            .takes_value(false))
        .arg(Arg::with_name("PATH")
            .help("Which path to parse")
            .required(true)
            .index(1))
        .get_matches();
    let os_cpus = num_cpus::get();
    let n_jobs = matches.value_of("JOBS").unwrap_or(&os_cpus.to_string())
        .parse::<usize>().unwrap();
    //    if let Some(ref n_jobs) = matches.value_of("JOBS").unwrap_or("1") {
    //        println!("number of jobs: {}", n_jobs);
    //    }

    let search_path = matches.value_of("PATH").unwrap();
    info!("processing path `{}` with {} threads",
          Green.paint(search_path.to_string()),
          Green.paint(n_jobs.to_string())
    );

    let mut verbosity = 0;
    match matches.occurrences_of("VERBOSE") {
        0 => verbosity = 0,
        1 => verbosity = 1,
        2 => verbosity = 2,
        3 => verbosity = 3,
        _ => verbosity = 4
    }

    let mut watch = false;
    match matches.occurrences_of("WATCH") {
        1 => watch = true,
        _ => ()
    }

    AppConfig { jobs: n_jobs, path: search_path.to_string(), verbose: verbosity, watch: watch }
}