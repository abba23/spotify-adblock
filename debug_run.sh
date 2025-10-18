#!/bin/bash
for pid in $(pgrep -f spotify); do
    kill -9 $pid
done

# If you got CEF for testing
export CEF_ROOT="$PWD/cef_binary_137.0.19+g8a1c4ce+chromium-137.0.7151.121_linux64" && cargo check
CEF_ROOT="$PWD/cef_binary_137.0.19+g8a1c4ce+chromium-137.0.7151.121_linux64" cargo build --release --lib

LD_PRELOAD=./target/release/libspotifyadblock.so spotify --enable-features=useozoneplatform --ozone-platform=wayland
