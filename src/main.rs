use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;
use notify::{DebouncedEvent, Watcher, RecursiveMode, watcher};

struct Cli {
    repository: String,
    path: PathBuf,
    alias: String,
    watch: bool,
}

fn main() {
    let repository = std::env::args().nth(1).expect("no repository given");
    let path = std::env::args().nth(2).expect("no path (file or folder) given");
    let alias = std::env::args().nth(3).expect("no alias given");
    let watch: bool = match std::env::args().nth(4) {
        Some(v) => {
            if v == "--watch" {
                true
            } else {
                panic!("unexpected argument {}", v)
            }
        },
        None => false
    };

    let args = Cli {
        repository: repository,
        path: PathBuf::from(path),
        alias: alias,
        watch: watch,
    };

    let local_repository = format!("{}/.wsync/sync-repository", std::env::var("HOME").unwrap());
    if !Path::new(&local_repository).exists() {
        println!("{}", local_repository);
        Command::new("git")
                .arg("clone")
                .arg(&args.repository)
                .arg(&local_repository)
                .status()
                .unwrap();
    }

    if args.watch {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();
        watcher.watch(args.path, RecursiveMode::Recursive).unwrap();
        loop {
            match rx.recv() {
                Ok(event) => {
                    match event {
                        DebouncedEvent::Write(path) => sync(&path, &args.alias, &local_repository),
                        _ => (),
                    }
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    } else {
        sync(&args.path, &args.alias, &local_repository)
    }
}

fn sync(file_path: &PathBuf, alias: &String, local_repository: &String) {
    let path_to = format!("{}/{}", &local_repository, &alias);
    fs::copy(&file_path, path_to).unwrap();
    Command::new("git")
            .arg("-C")
            .arg(&local_repository)
            .arg("add")
            .arg(".")
            .status()
            .unwrap();

    Command::new("git")
            .arg("-C")
            .arg(&local_repository)
            .arg("commit")
            .arg("-m")
            .arg(format!("Sync {}", &alias))
            .status()
            .unwrap();

    Command::new("git")
            .arg("-C")
            .arg(&local_repository)
            .arg("push")
            .arg("origin")
            .arg("master")
            .status()
            .unwrap();
}
