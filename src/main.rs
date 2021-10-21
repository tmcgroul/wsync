use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;
use wsync::{meta, sync, Cli};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let cli = Cli::new(&args);

    let local_repository = format!("{}/.wsync/sync-repository", std::env::var("HOME")?);
    if !Path::new(&local_repository).exists() {
        Command::new("git")
            .args(["clone", &cli.repository, &local_repository])
            .status()?;
        if !meta::Meta::exists(&local_repository) {
            meta::Meta::create(&local_repository)?;
        }
    }

    if cli.watch {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(10))?;
        watcher.watch(cli.path, RecursiveMode::Recursive)?;
        loop {
            match rx.recv() {
                Ok(event) => {
                    if let DebouncedEvent::Write(path) = event {
                        sync(&path, &cli.alias, &local_repository).expect("Synchronization error");
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    } else {
        sync(&cli.path, &cli.alias, &local_repository).expect("Synchronization error");
    }
    Ok(())
}
