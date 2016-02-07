use fern;
use log;
use time;
use csv::{Reader, };
use ansi_term::Colour::{Yellow, Green, Red, White};


/// boilerplate code to setup logging with `fern`
pub fn parse_csv_file(filename: &str) {
	let mut reader = Reader::from_file(filename).unwrap()
		.delimiter(b';')
		.has_headers(false);

    for record in reader.decode() {
        let (s1, s2, s3): (String, String, String) = record.unwrap();
        debug!("{} {} {}", Yellow.paint(s1), Green.paint(s2), Yellow.paint(s3));
    }
}

