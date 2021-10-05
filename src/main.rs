use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;
use wsync::{meta, sync, Cli};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cli = Cli::new(&args);

    let local_repository = format!("{}/.wsync/sync-repository", std::env::var("HOME").unwrap());
    if !Path::new(&local_repository).exists() {
        Command::new("git")
            .arg("clone")
            .arg(&cli.repository)
            .arg(&local_repository)
            .status()
            .unwrap();
        if !meta::Meta::exists(&local_repository) {
            meta::Meta::create(&local_repository);
        }
    }

    if cli.watch {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();
        watcher.watch(cli.path, RecursiveMode::Recursive).unwrap();
        loop {
            match rx.recv() {
                Ok(event) => {
                    if let DebouncedEvent::Write(path) = event {
                        sync(&path, &cli.alias, &local_repository);
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    } else {
        sync(&cli.path, &cli.alias, &local_repository)
    }
}
