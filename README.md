# tracker

A simple tool to track time.

## Usage

```
$ tracker --help
Simple tool to do time tracking

Usage: tracker.exe [OPTIONS] <COMMAND>

Commands:
  add     Add a track entry
  view    Display tracking list entries
  delete  Remove entries from a tracking list
  help    Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  Path to a config file
  -h, --help             Print help
  -V, --version          Print version
```

## Install

You can either download the latest release builds form the [Releases page](https://github.com/shellshape/tracker/releases) or you can install it using cargo install.
```
cargo install --git https://github.com/shellshape/tracker
```