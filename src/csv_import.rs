use fern;
use log;
use time;
use csv::{Reader, };
use ansi_term::Colour::{Yellow, Green, Red, White};
use std::string::{String, };


// https://peteris.rocks/blog/serialize-any-object-to-a-binary-format-in-rust/
// https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/index.html#simple-example-of-tojson-usage
#[derive(RustcDecodable)]
struct Record {
    depotNr: u64,
    wpkn: String,
    name: String,
    // kurs: f64,
}


/// boilerplate code to setup logging with `fern`
pub fn parse_csv_file(filename: &str) {
	let mut reader = Reader::from_file(filename).unwrap()
		.delimiter(b';')
		.has_headers(false);

    for record in reader.decode() {
    	let record: Record = record.unwrap();
        // let (s1, s2, s3): (String, String, String) = record.unwrap();
        debug!("{} {} {}", Yellow.paint(record.depotNr.to_string()), Green.paint(record.wpkn), Yellow.paint(record.name));
    }
}

// http://stackoverflow.com/a/24878421
// http://stackoverflow.com/questions/26346154/rust-string-indexing-compare-stri-char
fn de_to_us(mut de_str: &str) {
    for (i, c) in de_str.chars().enumerate() {
    	if c == ',' {
			debug!("found a comma in {}", de_str);
    	}
    };
}