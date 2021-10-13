use std::path::Path;
use std::process::{Command, Stdio};
use std::{fs, thread, time};
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

#[test]
fn test_sync_local_state() {
    fs::create_dir("remote-test-repository").unwrap();
    Command::new("git")
        .args(["-C", "remote-test-repository", "init", "--bare"])
        .stdout(Stdio::null())
        .status()
        .unwrap();

    Command::new("git")
        .args(["clone", "remote-test-repository", "test-repository"])
        .stdout(Stdio::null())
        .status()
        .unwrap();
    fs::write("test-repository/meta.txt", "").unwrap();
    fs::write("test.txt", "").unwrap();
    let test = Path::new("test.txt");
    sync(test, "test", "test-repository/");

    Command::new("git")
        .args(["clone", "remote-test-repository", "test-repository2"])
        .stdout(Stdio::null())
        .status()
        .unwrap();
    thread::sleep(time::Duration::from_secs(1));
    fs::write("test2.txt", "test").unwrap();
    let test2 = Path::new("test2.txt");
    sync(test2, "test", "test-repository2/");

    sync(test, "test", "test-repository");
    let content = fs::read_to_string("test-repository/test").unwrap();

    fs::remove_file("test.txt").unwrap();
    fs::remove_file("test2.txt").unwrap();
    fs::remove_dir_all("remote-test-repository").unwrap();
    fs::remove_dir_all("test-repository").unwrap();
    fs::remove_dir_all("test-repository2").unwrap();

    assert_eq!(content, "test");
}
