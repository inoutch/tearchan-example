#!/bin/bash

APP_NAME="tearchan-example-android"

set -e

rm -r target/debug/apk

cargo apk run --features vulkan || echo "Suppress signing error"

# Issue for https://github.com/rust-windowing/android-ndk-rs/issues/76#issuecomment-698508327
java -jar uber-apk-signer-1.1.0.jar --apks ./target/debug/apk/

adb install -r "./target/debug/apk/$APP_NAME-aligned-debugSigned.apk"

#adb logcat | grep tearchan_gfx
