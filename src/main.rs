extern crate notify;
use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::convert::AsRef;
use std::path::Path;

fn watch<P: AsRef<Path>> (path: P) -> notify::Result<()> {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = notify::watcher(tx, Duration::from_millis(100))?;
    // let mut watcher: RecommendedWatcher = notify::raw_watcher(tx)?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }
}


fn main() {
    if let Err(e) = watch("/Users/akilroy/workspace/non-ts/rust/fslisten") {
        eprintln!("error: {:?}", e)
    }
}

