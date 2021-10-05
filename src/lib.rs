use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

pub mod meta;

pub struct Cli {
    pub repository: String,
    pub path: PathBuf,
    pub alias: String,
    pub watch: bool,
}

impl Cli {
    pub fn new(args: &[String]) -> Cli {
        let repository = args.get(1).expect("no repository given").clone();
        let path = args.get(2).expect("no path (file or folder) given");
        let alias = args.get(3).expect("no alias given").clone();
        let watch = match args.get(4) {
            Some(value) => {
                if value == "--watch" {
                    true
                } else {
                    panic!("unexpected argument {}", value)
                }
            }
            None => false,
        };
        Cli {
            path: PathBuf::from(path),
            repository,
            alias,
            watch,
        }
    }
}

pub fn sync(file_path: &Path, alias: &str, local_repository: &str) {
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
