#!/bin/bash
# This file is responsible for compiling Horizon in the most optimizations possible.
set -e # exit on error

echo "Building initial instrumentation build"
cargo pgo run
echo "TODO: also start a process that will act as real players and do what real players would do"
echo "Optimizing binary"
cargo pgo optimize

# TODO: set up BOLT
# this is pretty hectic to set up for now, as it'd require you to compile it from source for the time being.
# not to mention, cargo-pgo is still experimenting with BOLT.
# maybe wait till later

