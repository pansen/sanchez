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
mod arguments;


use std::process::{exit, };
use ansi_term::Colour::{Yellow, Green, Red};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};
use std::path::{Path};
use id3::Tag;

fn main() {
    logging::setup_logging();

    let config = arguments::parse();
    let n_jobs = config.jobs;
    let base_path = config.path;


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
          Yellow.paint(path::realpath(Path::new(&base_path)).to_str().unwrap()));
    let walker = WalkDir::new(&base_path).into_iter();
    let pool = ThreadPool::new(n_jobs);
    let (tx, rx) = channel();
    let mut counter = 0;

    for file_ in walker.filter_entry(|e| e.path().is_dir() || (!is_hidden(e) && is_mp3(e))) {
        let file_ = file_.unwrap();
        if !file_.path().is_dir() {
            let tx = tx.clone();
            counter += 1;

            pool.execute(move || {
                match Tag::read_from_path(file_.path()) {
                    Err(why) => warn!("{:?}, failed to read: {:?}", why, file_.path()),
                    Ok(tag) => {
                        match tag.artist() {
                            None => warn!("failed to extract artist: {:?}", file_.path()),
                            Some(a_name) => {
                                // only count successful ones
                                debug!("{} recursed file from: {} - {} {} {}",
                                       Yellow.paint(counter.to_string()),
                                       Green.paint(a_name),
                                       Green.paint(tag.title().unwrap()),
                                       Red.paint(tag.album().unwrap()),
                                       file_.path().display());
                                tx.send(a_name.to_owned()).unwrap();
                            }
                        }
                    },
                };
                drop(tx);
            });
        }
    }
    drop(tx);

    for value in rx.iter() {
        debug!("receiving {} from thread", Green.paint(value));
    }
    exit(0);
}

