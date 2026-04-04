# Quickstart: 5G Modem调试工具

**Date**: 2026-04-04
**Feature**: 001-modem-debug-tool

## Installation

### Prerequisites

- Bun 1.0+ installed
- For desktop: Tauri CLI dependencies (Rust 1.70+, system development libraries)

### Install from Source

```bash
# Clone the repository
git clone https://github.com/yourorg/modem-cat.git
cd modem-cat

# Install dependencies
bun install

# Build the application
bun run build

# For desktop build
bun run build:desktop
```

## CLI Usage

### Quick Start

```bash
# List available serial ports
modem-cat list-ports

# Connect to modem via USB Serial
modem-cat connect --type usb --port /dev/cu.usbserial-1420

# Check modem status
modem-cat status

# Send AT command
modem-cat at "AT+CSQ"

# Get network info
modem-cat network-info

# Get hardware info
modem-cat hardware-info

# Disconnect
modem-cat disconnect
```

### Connection Examples

#### USB Serial

```bash
modem-cat connect --type usb --port /dev/cu.usbserial-1420 --baud 115200
```

#### Ethernet (TCP)

```bash
modem-cat connect --type ethernet --host 192.168.1.100 --port 9000
```

#### TTL (UART)

```bash
modem-cat connect --type ttl --port /dev/cu.usbserial-1420 --baud 9600
```

### Output Formats

```bash
# JSON output (default for scripts)
modem-cat status --json

# Human-readable output (default for terminal)
modem-cat status --human
```

### Command History

```bash
# View command history
modem-cat history

# Re-run previous command
modem-cat at "AT+CSQ"
```

### Batch AT Commands

```bash
# Execute commands from file
modem-cat at-script commands.txt
```

## Desktop Usage

### Launch

```bash
# Launch desktop application
modem-cat gui
```

### Interface Overview

1. **Connection Panel**: Select connection type, configure parameters, connect/disconnect
2. **Status Panel**: Display real-time modem status
3. **AT Command Panel**: Send commands and view responses
4. **Network Info**: Detailed cellular network information
5. **Hardware Info**: Modem hardware and system information
6. **Settings**: Configure application preferences

## Common Use Cases

### Check Signal Quality

```bash
# Quick signal check
modem-cat at "AT+CSQ"

# Detailed signal info
modem-cat network-info
```

### Configure Network Mode

```bash
# Set to 5G SA mode
modem-cat config set network-mode 5G

# Set to 4G only
modem-cat config set network-mode 4G

# View current config
modem-cat config get
```

### Firmware Information

```bash
# Get modem model and firmware
modem-cat at "ATI"

# Get hardware details
modem-cat hardware-info
```

## Troubleshooting

### Connection Issues

- **Port not found**: Check USB cable, try different port
- **Permission denied**: Add user to dialout group (Linux) or run as admin
- **Connection timeout**: Check baud rate, try different rate

### AT Command Issues

- **No response**: Check connection status, try AT command
- **ERROR response**: Check command syntax, consult modem documentation

### Output Issues

- **Garbled output**: Check baud rate matches modem settings
- **JSON parse error**: Check --json flag usage
