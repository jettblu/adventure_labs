## Getting Started

Before running thius project, install required system dependencies. 
```bash
sudo apt-get install -y libcamera-dev
sudo apt-get install -y libclang-dev
```

## Run Examples

```bash
cargo run -p motion_detection --bin motion_detection_loop --release
```

```bash
cargo run -p camera_capture --bin snapshot --release
```

```bash
cargo run -p smart_cam --bin run_smart_cam  --release
```

