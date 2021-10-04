use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime};

mod meta;

struct Cli {
    repository: String,
    path: PathBuf,
    alias: String,
    watch: bool,
}

fn main() {
    let repository = std::env::args().nth(1).expect("no repository given");
    let path = std::env::args()
        .nth(2)
        .expect("no path (file or folder) given");
    let alias = std::env::args().nth(3).expect("no alias given");
    let watch: bool = match std::env::args().nth(4) {
        Some(v) => {
            if v == "--watch" {
                true
            } else {
                panic!("unexpected argument {}", v)
            }
        }
        None => false,
    };

    let args = Cli {
        path: PathBuf::from(path),
        repository,
        alias,
        watch,
    };

    let local_repository = format!("{}/.wsync/sync-repository", std::env::var("HOME").unwrap());
    if !Path::new(&local_repository).exists() {
        Command::new("git")
            .arg("clone")
            .arg(&args.repository)
            .arg(&local_repository)
            .status()
            .unwrap();
        if !meta::Meta::exists(&local_repository) {
            meta::Meta::create(&local_repository);
        }
    }

    if args.watch {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();
        watcher.watch(args.path, RecursiveMode::Recursive).unwrap();
        loop {
            match rx.recv() {
                Ok(event) => {
                    if let DebouncedEvent::Write(path) = event {
                        sync(&path, &args.alias, &local_repository);
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    } else {
        sync(&args.path, &args.alias, &local_repository)
    }
}

fn sync(file_path: &Path, alias: &str, local_repository: &str) {
    let metadata = fs::metadata(&file_path).unwrap();
    let modified = metadata
        .modified()
        .unwrap()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let meta = meta::Meta::from(local_repository);
    meta.update(alias, modified);

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
