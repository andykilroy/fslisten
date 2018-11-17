extern crate notify;
use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::convert::AsRef;
use std::path::Path;

extern crate clap;
use clap::{Arg, App, SubCommand};

fn watch<P: AsRef<Path>> (path: P) -> notify::Result<()> {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = notify::watcher(tx, Duration::from_millis(100))?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }
}


fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Listen for operations on files and directories")
        .arg(Arg::with_name("FILE")
                      .help("The file or directory to monitor")
                      .required(true)
                      .index(1))
        .get_matches();

    let target = matches.value_of("FILE").expect("expected a file or directory on which to listen for changes");
    if let Err(e) = watch(target) {
        eprintln!("error: {:?}", e)
    }

}

