## Getting Started

Before running thius project, install required system dependencies. 
```bash
sudo apt-get install -y libcamera-dev
sudo apt-get install -y libclang-dev
```

## Add Requirments 
Install pisugar battery application.
```bash
wget https://cdn.pisugar.com/release/pisugar-power-manager.sh
bash pisugar-power-manager.sh -c release
```

Navigate to root and create required directories.
```bash
cd /
sudo mkdir rugged_cam
cd rugged_cam
sudo mkdir logs
```

Install required packages for meshtastic.

```bash
sudo apt install libdbus-1-dev pkg-config
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

## Helpful Commands

**Pisugar**
```bash

# get battery
echo "get battery" | nc -q 0 127.0.0.1 8423

# get rtc time
echo "get rtc_time" | nc -q 0 127.0.0.1 8423

# reload daemon
sudo systemctl daemon-reload

# check status
sudo systemctl status pisugar-server

# start service
sudo systemctl start pisugar-server

# stop service
sudo systemctl stop pisugar-server

# disable service
sudo systemctl disable pisugar-server

# enable service
sudo systemctl enable pisugar-server
```

**Linux Automation**
```bash
# list all services on computer
systemctl list-units --type=service

# view system file contents 
systemctl cat <fle_name>

# copy service file to systemd
sudo cp <service file path> /etc/systemd/system/

# copy binary file to rugged cam binary directory
sudo cp <service file path> /usr/bin/rugged_cam

# remove file
sudo rm <file_name>
```

**Logs**
```bash
# enter rugged cam logs directory
cd /etc/rugged_cam/logs

```




