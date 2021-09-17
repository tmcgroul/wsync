# wsync
A tool for workplace syncronization. It uses git as a version control system to track changes

# Usage
You can run `wsync` in two modes:
### sync current state and exit
```sh
cargo run <git-repository path> <file/folder-to-sync path>
```
### sync current state and watch changes
```sh
cargo run <git-repository path> <file/folder-to-sync path> --watch
```

# Example
```sh
cargo run git@github.com:tmcgroul/workplace-config.git /home/sar/.config/terminator/config
```
