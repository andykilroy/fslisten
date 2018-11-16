extern crate notify;
use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;

fn watch() -> notify::Result<()> {
    let (tx, rx) = channel();
    // let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
    let mut watcher: RecommendedWatcher = notify::raw_watcher(tx)?;

    watcher.watch("/Users/akilroy/workspace/non-ts/rust/fslisten", RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn main() {
    if let Err(e) = watch() {
        println!("error: {:?}", e)
    }
}

