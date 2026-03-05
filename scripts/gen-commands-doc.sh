#!/bin/bash

set -ex

currdir=$(realpath "$(dirname "$0")")
RUSTFLAGS=-Awarnings cargo run --quiet --features clap-markdown > "$currdir/../docs/commands.md"
