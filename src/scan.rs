use path;
use arguments::AppConfig;
use ansi_term::Colour::{Yellow, Green, Red};
use threadpool::ThreadPool;
use std::sync::mpsc;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};
use std::path::{Path};
use id3::Tag;
use crypto::digest::Digest;
use crypto::sha2::Sha512;
use std::io::prelude::*;
use std::fs::File;
use models;
use manager::TrackManager;


/// struct which creates an object acting as a scanner. we'll hold only one of this
/// to take advantage of a shared `ThreadPool`, preconfigured `base_path` etc.
pub struct Scanner<'a> {
    base_path: String,
    thread_pool: ThreadPool,
    thread_number: usize,
    track_manager: &'a TrackManager<'a>,
}

impl<'a> Scanner<'a> {
    /// constructor
    pub fn new(config: &AppConfig, manager: &'a TrackManager) -> Scanner<'a> {
        Scanner {
            base_path: config.path.to_owned(),
            thread_pool: ThreadPool::new(config.jobs),
            thread_number: config.jobs,
            track_manager: manager
        }
    }

    /// search a given path for mp3 files
    pub fn scan_all(&self) {
        info!("searching for files in `{}`",
              Yellow.paint(path::realpath(Path::new(&self.base_path)).to_str().unwrap()));
        let walker = WalkDir::new(&self.base_path).into_iter();
        let (tx, rx) = mpsc::channel();

        for file_ in walker.filter_entry(|e| e.path().is_dir() || (!is_hidden(e) && is_mp3(e))) {
            let file_ = file_.unwrap();
            if !file_.path().is_dir() {
                let tx = tx.clone();

                self.thread_pool.execute(move || {
                    extract_tag(&file_.path(), &tx)
                });
            }
        }
        drop(tx);
        for track_ in rx.iter() {
            info!("{:?}", &track_.path);
            warn!("{} - {}  [{}]",
                  Green.paint(track_.album.to_owned()),
                  Green.paint(track_.title.to_owned()),
                  track_.hash.to_owned(),
            );
            self.track_manager.create_track(&track_.path, &track_.album, &track_.title,
                                            &track_.hash);
        }
    }

    /// re-scan a given file
    /// TODO amb: merge the threaded part to be in one place from `scan_all` and `scan_file`
    pub fn scan_file(&self, path_name: &str) {
        info!("scan single file `{}`", Yellow.paint(path_name));
        let path_name_copy = path_name.to_owned();
        let (tx, rx) = mpsc::channel();

        self.thread_pool.execute(move || {
            let path = Path::new(&path_name_copy);
            if !path.is_dir() {
                extract_tag(&path, &tx);
            }
        });

        for value in rx.iter() {
            warn!("[single] receiving {} - {} from thread",
                  Red.paint(value.album),
                  Green.paint(value.title),
            );
        }
    }
}

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

/// encapsulates the tag-extraction logic
fn extract_tag(path: &Path, tx_: &mpsc::Sender<models::Track>) {
    let mut sha = Sha512::new();
    let mut f = File::open(path).unwrap();
    let mut buffer = Vec::new();

    let _ = f.read_to_end(&mut buffer);
    sha.input(buffer.as_slice());

    let hash = sha.result_str();

    match Tag::read_from_path(path) {
        Err(why) => {
            error!("{:?}, failed to read: {:?}", why, path);
            let found = models::Track {
                path: path.display().to_string(),
                title: path::basename(path).display().to_string(),
                album: "".to_string(),
                hash: hash.to_owned()
            };
            tx_.send(found).unwrap();
        },
        Ok(tag) => {
            match tag.title() {
                None => warn!("failed to extract title: {:?}", path),
                Some(track_title) => {
                    let track_album = tag.album().unwrap();
                    debug!("extracted file: {}", path.display());
                    let found = models::Track {
                        path: path.display().to_string(),
                        title: track_title.to_owned(),
                        album: track_album.to_owned(),
                        hash: hash.to_owned()
                    };
                    tx_.send(found).unwrap();
                }
            }
        }
    };
    drop(tx_);
}