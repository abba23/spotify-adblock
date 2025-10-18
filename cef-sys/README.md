# cef-sys

Raw FFI bindings for Chromium Embedded Framework (CEF).

## Overview

This crate provides low-level Rust bindings to the CEF C API, generated using bindgen. These bindings are used internally by the spotify-adblock project to hook into CEF's networking functions.

## Requirements

- CEF binary distribution (downloaded from https://cef-builds.spotifycdn.com/)
- CEF_ROOT environment variable pointing to the CEF distribution directory

## Building

```bash
export CEF_ROOT=/path/to/cef_binary_137.0.19+g8a1c4ce+chromium-137.0.7151.121_linux64
cargo build
```

## Usage

This crate is typically not used directly. It provides the raw FFI bindings that higher-level crates build upon.
