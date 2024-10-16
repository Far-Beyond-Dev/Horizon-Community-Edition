#!/bin/sh
# ..by the way

set -e
sudo pacman -Syu rustup clang sqlite openssl go curl clang gcc-multilib --needed

rustup toolchain install stable

