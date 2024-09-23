#!/bin/bash
# This file is responsible for setting up a proper environment for compiling Horizon with docker.
set -e # exit on error

echo "Installing cargo-pgo"
# cargo install cargo-pgo --locked

echo "Installing llvm-tools-preview"
# rustup component add llvm-tools-preview

if [ -f "./installers.sh" ]; then
    echo "If you are running a Linux distro that uses apt as it's package manager, you may want to run the ./installers.sh file"
    echo "Do that now? [y/N] "
    read installrs
    if [ "$installrs" = "y" ]; then
        chmod +x ./installers.sh
        ./installers
    fi
else
    echo "Can't find installers.sh!"
fi

