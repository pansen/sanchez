#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

pub mod models;
pub mod schema;

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
extern crate num_cpus;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

mod logging;
mod path;
mod arguments;
mod scan;
mod watch;
mod manager;

use std::process::{exit, };
use std::thread;
use std::vec::Vec;
use ansi_term::Colour::{Yellow};

use diesel::prelude::*;

use models::{Track, NewTrack};


fn main() {
    let config = arguments::parse();
    logging::setup_logging(&config);
    info!("running with {} threads, connection: {}", Yellow.paint(config.jobs.to_string()),
          Yellow.paint(config.database_url.to_owned()));

    {
        let pool = config.pool.clone();

        thread::spawn(move || {
            let connection = &*pool.get().unwrap();
            let track_manager = manager::TrackManager::new(connection);
            let created_track = track_manager
                .create_track("path",
                              "title",
                              "album",
                              &time::strftime("%Y-%m-%d %H:%M:%S.%f", &time::now()).unwrap());

            info!("created track: {} - {}  [{}]",
                  Yellow.paint(created_track.album),
                  Yellow.paint(created_track.title),
                  created_track.hash);

            track_manager.show_tracks();
            // TODO amb: won't work:
            //            thread::spawn(move || {
            //                track_manager.show_tracks();
            //            });
        });
    }

    let mut watcher_handles: Vec<thread::JoinHandle<_>> = Vec::with_capacity(1);

    if config.watch == true {
        let config = config.clone();

        watcher_handles.push(thread::spawn(move || {
            let connection = &*config.pool.get().unwrap();
            let track_manager = manager::TrackManager::new(connection);
            // TODO amb: would be nicer to share only *one* `Scanner` here, but i don't know how exactly
            let scanner_thread = scan::Scanner::new(&config, &track_manager);

            let _ = watch::watch_reference(&config, &scanner_thread);
        }));
    }
    let connection = &*config.pool.get().unwrap();
    let track_manager = manager::TrackManager::new(connection);
    track_manager.show_tracks();

    let scanner = scan::Scanner::new(&config, &track_manager);
    // connection_pool: r2d2::Pool<r2d2_diesel::ConnectionManager<diesel::sqlite::SqliteConnection>>
    scanner.scan_all();

    // after the scanner is done, we just join the watcher thread to avoid the mainthread to exit
    // creating a channel could also be done, but seems like overhead here
    // http://stackoverflow.com/a/26200583/2741111
    for handle in watcher_handles {
        let _ = handle.join();
    }

    exit(0);
}

