# wsync
A tool for workplace syncronization. It uses git as a version control system to track changes

# Usage
You can run `wsync` in two modes:
### sync current state and exit
```sh
cargo run <git-repository path> <file/folder-to-sync path> <alias>
```
### sync current state and watch changes
```sh
cargo run <git-repository path> <file/folder-to-sync path> <alias> --watch
```

# Example
```sh
cargo run git@github.com:tmcgroul/workplace-config.git /home/sar/.config/terminator/config terminator
```

# Development
It's required to install [pre-commit](https://pre-commit.com/#3-install-the-git-hook-scripts) after git clone
```sh
pre-commit install
```
