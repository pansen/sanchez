use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use diesel;
use ansi_term::Colour::{Yellow};

use models::{Track, NewTrack};

// this refers to the `Track` tablename
use schema::track;
use schema::track::dsl::track as track_dsl;


/// manager struct for the `Track` entity
///
/// note: the `<'a>` defines the lifetime: https://doc.rust-lang.org/book/lifetimes.html#impl-blocks
pub struct TrackManager<'a> {
    conn: &'a SqliteConnection,
}

impl<'a> TrackManager<'a> {
    pub fn new(conn: &'a SqliteConnection) -> TrackManager {
        TrackManager {
            conn: conn
        }
    }

    pub fn create_track<'b>(&self, path: &'b str, title: &'b str, album: &'b str, hash: &'b str) -> Track {
        let new_track = NewTrack {
            path: path,
            title: title,
            album: album,
            hash: hash,
        };

        match diesel::insert(&new_track).into(track::table)
            .execute(self.conn) {
            Err(why) => {
                error!("failed saving new track: {:?}, {:?}", why, new_track);
            },
            Ok(t_) => {
                debug!("saved new track: {:?}", t_);
            }
        }

        track_dsl.find(hash).get_result::<Track>(self.conn)
            .expect(&format!("Unable to find track {}", hash))
    }

    pub fn by_path<'b>(&self, path: &'b str) -> Result<Track, diesel::result::Error> {
        track_dsl.filter(track::path.eq(path)).get_result::<Track>(self.conn)
    }

    pub fn by_hash<'b>(&self, hash: &'b str) -> Result<Track, diesel::result::Error> {
        track_dsl.find(hash).get_result::<Track>(self.conn)
    }

    pub fn show_tracks(&self) {
        // TODO amb: no idea what the `*` is doing here. but it solves a problem
        // see: https://github.com/diesel-rs/diesel/issues/339
        let results = track_dsl
            .load::<Track>(self.conn)
            .expect("Error loading tracks");
        info!("found {:?} tracks", results.len());
        for t_ in results {
            debug!("found track in db: {} - {}  [{}]",
                   Yellow.paint(t_.album),
                   Yellow.paint(t_.title),
                   t_.hash)
        }
    }
}
