use super::schema::track;


/// Channel struct for found tracks
#[derive(Queryable)]
pub struct Track {
    /// path of that file
    pub path: String,
    /// id3 title
    pub title: String,
    /// id3 album
    pub album: String,
    /// hash of the parsed file
    pub hash: String
}


#[insertable_into(track)]
pub struct NewTrack<'a> {
    pub path: &'a str,
    pub title: &'a str,
    pub album: &'a str,
    pub hash: &'a str,
}