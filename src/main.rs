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

use std::process::{exit, };
use std::thread;
use std::vec::Vec;
use ansi_term::Colour::{Yellow};

use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;
use diesel::prelude::*;
use self::models::{Track, NewTrack};


pub fn create_track<'a>(conn: &SqliteConnection) -> Track {
    use schema::track;
    use schema::track::dsl::track as track_dsl;

    let path = "path";
    let title = "title";
    let album = "album";
    let hash = "hash";

    let new_track = NewTrack {
        path: path,
        title: title,
        album: album,
        hash: hash,
    };

    diesel::insert(&new_track).into(track::table)
        .execute(conn)
        .expect("Error saving new post");

    track_dsl.find(hash)
        .get_result::<Track>(conn)
        .expect(&format!("Unable to find track {}", hash))
}


fn main() {
    let config = arguments::parse();
    logging::setup_logging(&config);
    info!("running with {} threads, connection: {}", Yellow.paint(config.jobs.to_string()),
          Yellow.paint(config.database_url.to_owned()));

    // this refers to the `Track` tablename
    use schema::track::dsl::track as track_dsl;

    let r2d2_config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(config.database_url.to_owned());
    let pool = r2d2::Pool::new(r2d2_config, manager).expect("Failed to create pool.");

    let created_track = create_track(&*pool.get().unwrap());
    info!("created track: {} - {}  [{}]",
          Yellow.paint(created_track.album),
          Yellow.paint(created_track.title),
          created_track.hash);

    // TODO amb: no idea what the `*` is doing here. but it solves a problem
    // see: https://github.com/diesel-rs/diesel/issues/339
    let results = track_dsl
        .limit(5)
        .load::<models::Track>(&*pool.get().unwrap())
        .expect("Error loading tracks");
    info!("found {:?} tracks", results.len());
    for t_ in results {
        info!("found track in db: {} - {}  [{}]",
              Yellow.paint(t_.album),
              Yellow.paint(t_.title),
              t_.hash)
    }

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

