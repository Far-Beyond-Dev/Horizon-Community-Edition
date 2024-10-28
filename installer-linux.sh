#!/bin/bash
if command -v apt-get >/dev/null 2>&1; then
    echo "Ubuntu Detected"
    sudo apt-get update
    sudo apt-get install clang libsqlite3-dev libssl-dev build-essentials rustup libclang-dev gcc-multilib libsqlite3-dev libclang-dev -y
    rustup toolchain install stable
elif command -v pacman >/dev/null 2>&1; then
    echo "Arch Linux Detected"
    sudo pacman -Syu rustup clang sqlite openssl clang gcc-multilib --needed
    rustup toolchain install stable
elif command -v apk >/dev/null 2>&1; then
    echo "Alpine Linux Detected"
    sudo apk update
    sudo apk add clang sqlite-dev openssl-dev build-base rustup curl libclang gcc-multilib
    rustup toolchain install stable
else
    echo "Unsupported package manager"
    exit 1
fi

echo "Installation complete!"
