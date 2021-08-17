use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

struct Cli {
    repository: String,
    path: PathBuf,
}

fn main() {
    let repository = std::env::args().nth(1).expect("no repository given");
    let path = std::env::args().nth(2).expect("no path (file or folder) given");

    let args = Cli {
        repository: repository,
        path: PathBuf::from(path),
    };

    if !Path::new("./sync-repository").exists() {
        Command::new("git")
                .arg("clone")
                .arg(&args.repository)
                .arg("sync-repository")
                .status()
                .unwrap();
    }

    let file_name = args.path.file_name().clone().unwrap().to_str().unwrap();
    let path_to = format!("./sync-repository/{}", file_name);
    let path_from = args.path.to_str().unwrap().clone();
    fs::copy(path_from, path_to).unwrap();
    Command::new("git")
            .arg("-C")
            .arg("./sync-repository")
            .arg("add")
            .arg(".")
            .status()
            .unwrap();

    Command::new("git")
            .arg("-C")
            .arg("./sync-repository")
            .arg("commit")
            .arg("-m")
            .arg(format!("Sync {}", file_name))
            .status()
            .unwrap();

    Command::new("git")
            .arg("-C")
            .arg("./sync-repository")
            .arg("push")
            .arg("origin")
            .arg("master")
            .status()
            .unwrap();
}
