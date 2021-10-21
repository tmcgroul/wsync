use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, SystemTimeError};
use std::{fmt, fs, io};

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

pub fn sync(file_path: &Path, alias: &str, local_repository: &str) -> Result<(), SyncError> {
    Command::new("git")
        .args(["-C", local_repository, "pull", "origin", "master"])
        .stdout(Stdio::null())
        .status()?;

    let metadata = fs::metadata(&file_path)?;
    let modified = metadata
        .modified()?
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let meta = meta::Meta::new(local_repository);
    let remote_modified = meta.get(alias)?;

    let repository_file_path = format!("{}/{}", &local_repository, &alias);
    if remote_modified.is_some() && remote_modified.unwrap() > modified {
        fs::copy(repository_file_path, &file_path)?;
    } else {
        meta.update(alias, modified)?;
        fs::copy(&file_path, repository_file_path)?;

        Command::new("git")
            .args(["-C", local_repository, "add", "."])
            .status()?;

        let message = format!("Sync {}", &alias);
        Command::new("git")
            .args(["-C", local_repository, "commit", "-m", &message])
            .stdout(Stdio::null())
            .status()?;

        Command::new("git")
            .args(["-C", local_repository, "push", "origin", "master"])
            .stdout(Stdio::null())
            .status()?;
    }
    Ok(())
}

#[derive(Debug)]
pub enum SyncError {
    IoError(io::Error),
    ReadError(meta::ReadError),
    SystemTimeError(SystemTimeError),
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cringe")
    }
}

impl From<io::Error> for SyncError {
    fn from(error: io::Error) -> Self {
        SyncError::IoError(error)
    }
}

impl From<meta::ReadError> for SyncError {
    fn from(error: meta::ReadError) -> Self {
        SyncError::ReadError(error)
    }
}

impl From<SystemTimeError> for SyncError {
    fn from(error: SystemTimeError) -> Self {
        SyncError::SystemTimeError(error)
    }
}
