extern crate notify;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use notify::DebouncedEvent::*;
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
            Ok(ev) => print_debounced(ev),
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }
}

fn print_debounced(event: DebouncedEvent) {
    match event {
        NoticeWrite(p)  => println!("write(1) {}", p.to_str().unwrap_or("")),
        NoticeRemove(p) => println!("rm(1)    {}", p.to_str().unwrap_or("")),
        Create(p)       => println!("create   {}", p.to_str().unwrap_or("")),
        Write(p)        => println!("write    {}", p.to_str().unwrap_or("")),
        Chmod(p)        => println!("chmod    {}", p.to_str().unwrap_or("")),
        Remove(p)       => println!("rm       {}", p.to_str().unwrap_or("")),
        Rename(p, q)    => println!("mv       {} {}", p.to_str().unwrap_or(""), q.to_str().unwrap_or("")),
        Rescan          => println!("rescan"),
        Error(err, opt) => println!("error    {}, {:?}", err, opt)
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

