use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;
use diesel::prelude::*;
use diesel;
use ansi_term::Colour::{Yellow};

use models::{Track, NewTrack};
// this refers to the `Track` tablename
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
        use schema::track;
        use schema::track::dsl::track as track_dsl;

        let new_track = NewTrack {
            path: path,
            title: title,
            album: album,
            hash: hash,
        };

        diesel::insert(&new_track).into(track::table)
            .execute(self.conn)
            .expect("Error saving new track");

        track_dsl.find(hash)
            .get_result::<Track>(self.conn)
            .expect(&format!("Unable to find track {}", hash))
    }

    pub fn show_tracks(&self) {
        // TODO amb: no idea what the `*` is doing here. but it solves a problem
        // see: https://github.com/diesel-rs/diesel/issues/339
        let results = track_dsl
            .limit(5)
            .load::<Track>(self.conn)
            .expect("Error loading tracks");
        info!("found {:?} tracks", results.len());
        for t_ in results {
            info!("found track in db: {} - {}  [{}]",
                  Yellow.paint(t_.album),
                  Yellow.paint(t_.title),
                  t_.hash)
        }
    }
}
