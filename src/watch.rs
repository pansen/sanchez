use notify;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::sync_channel;
use std::thread;
use notify::{RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;
use arguments::AppConfig;
use ansi_term::Colour::{Green, Yellow};


/// watcher function to check one directory for changes
pub fn watch_reference(config: &AppConfig) -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx));

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    try!(watcher.watch(config.path.to_owned()));
    warn!("watching directory: {}", Green.paint(config.path.to_owned()));

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(notify::Event { path: Some(path), op: Ok(op) }) => {
                info!("{:?} {:?}", op, path);
            },
            Err(e) => error!("watch error {}", e),
            _ => ()
        }
    }
}