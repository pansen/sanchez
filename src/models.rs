use super::schema::track;


/// Channel struct for found tracks
///
/// TODO amb: *attention* the `PRIMARY KEY` field must be the first in this struct
/// #[column_name(something)] will not work
#[derive(Debug)]
#[derive(Queryable)]
pub struct Track {
    /// hash of the parsed file
    pub hash: String,
    /// path of that file
    pub path: String,
    /// id3 title
    pub title: String,
    /// id3 album
    pub album: String,
}


#[derive(Debug)]
#[insertable_into(track)]
pub struct NewTrack<'a> {
    pub hash: &'a str,
    pub path: &'a str,
    pub title: &'a str,
    pub album: &'a str,
}