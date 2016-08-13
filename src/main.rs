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


mod logging;
mod path;


use std::process::{exit, };
use ansi_term::Colour::{Yellow, Green, Red, White};
use clap::{App, Arg};
use dotenv::dotenv;
use std::env;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread;
use std::fs;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};
use std::path::{Path};
use id3::Tag;

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
    info!("processing {} jobs with {} threads", Green.paint(n_jobs.to_string()),
          Green.paint(n_workers.to_string()));


    let base_path = "/Users/andi/Dropbox";

    // flat hirachy listing
    let paths = fs::read_dir(base_path).unwrap();

    for path in paths {
        info!("flat file: {}", path.unwrap().path().display())
    }


    // recursive listing
    fn is_hidden(entry: &DirEntry) -> bool {
        if entry.path().is_dir() {
            false
        } else {
            // let real = path::realpath(entry.path());
            let base = path::basename(entry.path());
            // debug!("check realpath: `{}`, basename: `{}`", real.to_str().unwrap(),
            //       base.to_str().unwrap());
            base.to_str()
                .map(|s| s.starts_with("."))
                .unwrap_or(false)
        }
    }
    fn is_mp3(entry: &DirEntry) -> bool {
        let base = path::basename(entry.path());
        base.to_str()
            .map(|s| s.ends_with(".mp3"))
            .unwrap_or(false)
    }

    info!("searching for files in `{}`",
          Yellow.paint(path::realpath(Path::new(base_path)).to_str().unwrap()));
    let walker = WalkDir::new(base_path).into_iter();
    for entry in walker.filter_entry(|e| e.path().is_dir() || (!is_hidden(e) && is_mp3(e))) {
        let entry = entry.unwrap();
        if !entry.path().is_dir() {
            let mut tag = Tag::read_from_path(entry.path()).unwrap();
            debug!("recursed file from: {} {}",
                   Green.paint(tag.artist().unwrap()), entry.path().display());
        }
    }


    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for job in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            debug!("sending {} from thread", Yellow.paint(job.to_string()));
            thread::sleep(Duration::from_millis(100));
            tx.send(job.to_string()).unwrap();
        });
    }

    for value in rx.iter().take(n_jobs) {
        debug!("receiving {} from thread", Green.paint(value));
    }
    exit(0);
}

