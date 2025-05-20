#!/bin/bash

TAPE_DIR=".github/media/demo.tape"

set +ex

rm -rf storage
vhs validate "$TAPE_DIR"
vhs "$TAPE_DIR" -o "${TAPE_DIR%.*}.gif"
