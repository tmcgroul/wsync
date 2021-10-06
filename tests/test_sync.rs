use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use wsync::sync;

#[test]
fn test_sync_repository_state() {
    fs::create_dir("remote-test-repository").unwrap();
    Command::new("git")
        .arg("-C")
        .arg("remote-test-repository")
        .arg("init")
        .arg("--bare")
        .stdout(Stdio::null())
        .status()
        .unwrap();

    Command::new("git")
        .arg("clone")
        .arg("remote-test-repository")
        .arg("test-repository")
        .stdout(Stdio::null())
        .status()
        .unwrap();
    fs::write("test-repository/meta.txt", "").unwrap();

    fs::write("test.txt", "").unwrap();
    let path = Path::new("test.txt");
    sync(path, "test", "test-repository/");

    Command::new("git")
        .arg("-C")
        .arg("test-repository/")
        .arg("pull")
        .stdout(Stdio::null())
        .status()
        .unwrap();
    assert!(Path::new("test-repository/test").exists());

    fs::remove_file("test.txt").unwrap();
    fs::remove_dir_all("remote-test-repository").unwrap();
    fs::remove_dir_all("test-repository").unwrap();
}
