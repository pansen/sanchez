use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;
use diesel::prelude::*;
use diesel;

use models::{Track, NewTrack};


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

    pub fn create_track(&self) -> Track {
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
            .execute(self.conn)
            .expect("Error saving new post");

        track_dsl.find(hash)
            .get_result::<Track>(self.conn)
            .expect(&format!("Unable to find track {}", hash))
    }
}
