# Release Binaries

This directory is for storing rugged cam release binaries used by the system.

## Setup

Build the binaries.
```bash
cargo build -p battery --bin cam-health --release
cargo build -p smart_cam --bin run_smart_cam --release
```
The binaries will be output in *target/release*. Copy the binaries into this folder.