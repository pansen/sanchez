use path;
use arguments::AppConfig;
use ansi_term::Colour::{Yellow, Green, Red};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};
use std::path::{Path};
use id3::Tag;


/// Channel struct for found tracks
pub struct FoundTrack {
    /// path of that file
    pub path: String,
    pub title: String,
    pub album: String
}


/// struct which creates an object acting as a scanner. we'll hold only one of this
/// to take advantage of a shared `ThreadPool`, preconfigured `base_path` etc.
pub struct Scanner {
    base_path: String,
    thread_pool: ThreadPool,
    thread_number: usize
}

impl Scanner {
    /// constructor
    pub fn new(config: &AppConfig) -> Scanner {
        Scanner {
            base_path: config.path.to_owned(),
            thread_pool: ThreadPool::new(config.jobs),
            thread_number: config.jobs
        }
    }

    /// search a given path for mp3 files
    pub fn scan_all(&self) {
        info!("searching for files in `{}`",
              Yellow.paint(path::realpath(Path::new(&self.base_path)).to_str().unwrap()));
        let walker = WalkDir::new(&self.base_path).into_iter();
        let (tx, rx) = channel();
        let mut counter = 0;

        for file_ in walker.filter_entry(|e| e.path().is_dir() || (!is_hidden(e) && is_mp3(e))) {
            let file_ = file_.unwrap();
            if !file_.path().is_dir() {
                let tx = tx.clone();
                counter += 1;

                self.thread_pool.execute(move || {
                    match Tag::read_from_path(file_.path()) {
                        Err(why) => {
                            warn!("{:?}, failed to read: {:?}", why, file_.path());
                            let found = FoundTrack {
                                path: file_.path().display().to_string(),
                                title: path::basename(file_.path()).display().to_string(),
                                album: "".to_string()
                            };
                            tx.send(found).unwrap();
                        },
                        Ok(tag) => {
                            match tag.title() {
                                None => warn!("failed to extract title: {:?}", file_.path()),
                                Some(track_title) => {
                                    let track_album = tag.album().unwrap();
                                    debug!("{} recursed file: {}",
                                           Yellow.paint(counter.to_string()),
                                           file_.path().display());
                                    let found = FoundTrack {
                                        path: file_.path().display().to_string(),
                                        title: track_title.to_owned(),
                                        album: track_album.to_owned()
                                    };
                                    tx.send(found).unwrap();
                                }
                            }
                        }
                    };
                    drop(tx);
                });
            }
        }
        drop(tx);

        for value in rx.iter() {
            debug!("receiving {} - {} from thread",
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

