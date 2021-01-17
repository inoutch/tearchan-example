#!/bin/sh
set -eux

PATH=$PATH:$HOME/.cargo/bin
cd ..
cargo build --target aarch64-apple-ios
