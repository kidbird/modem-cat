# CLI Command Interface

## Connection Commands

### connect

```bash
modem-cat connect [options]
```

**Options**:
- `--type <usb|ethernet|ttl>`: Connection type (required)
- `--port <path>`: Serial port path (for usb/ttl)
- `--baud <rate>`: Baud rate (default: 115200)
- `--host <ip>`: IP address (for ethernet)
- `--port <number>`: Port number (for ethernet)
- `--timeout <ms>`: Connection timeout in ms (default: 5000)

**Output**:
```json
{
  "success": true,
  "connection": {
    "id": "conn_abc123",
    "type": "USB_SERIAL",
    "status": "CONNECTED"
  }
}
```

### disconnect

```bash
modem-cat disconnect [connection-id]
```

**Output**:
```json
{
  "success": true,
  "message": "Disconnected from /dev/cu.usbserial-1420"
}
```

### list-ports

```bash
modem-cat list-ports
```

**Output**:
```json
{
  "success": true,
  "ports": [
    {
      "path": "/dev/cu.usbserial-1420",
      "description": "USB Serial Device",
      "manufacturer": "Silicon Labs"
    }
  ]
}
```

## Status Commands

### status

```bash
modem-cat status [connection-id]
```

**Output**:
```json
{
  "success": true,
  "status": {
    "registration": "REGISTERED",
    "signalStrength": {
      "rsrp": -85,
      "rsrq": -12
    },
    "connectionMode": "5G",
    "imei": "123456789012345",
    "operator": "China Mobile"
  }
}
```

### network-info

```bash
modem-cat network-info [connection-id]
```

**Output**:
```json
{
  "success": true,
  "network": {
    "operatorName": "China Mobile",
    "plmn": "46000",
    "band": "n78",
    "bandwidth": 100,
    "pci": 123,
    "rsrp": -85,
    "rsrq": -12,
    "snr": 15
  }
}
```

### hardware-info

```bash
modem-cat hardware-info [connection-id]
```

**Output**:
```json
{
  "success": true,
  "hardware": {
    "model": "MF269",
    "firmwareVersion": "V3.2.1",
    "hardwareVersion": "Rev.3",
    "manufacturer": "Fibocom",
    "cpuUsage": 25,
    "memoryUsage": 45,
    "uptime": 86400
  }
}
```

## AT Command Commands

### at

```bash
modem-cat at <command> [connection-id]
```

**Example**:
```bash
modem-cat at "AT+CSQ" conn_abc123
```

**Output**:
```json
{
  "success": true,
  "command": {
    "id": "cmd_xyz789",
    "command": "AT+CSQ",
    "response": "+CSQ: 20,99\n\nOK",
    "status": "OK",
    "duration": 45
  }
}
```

### at-script

```bash
modem-cat at-script <file> [connection-id]
```

**Output**:
```json
{
  "success": true,
  "results": [
    { "command": "ATI", "response": "...", "status": "OK" },
    { "command": "AT+CGMR", "response": "...", "status": "OK" }
  ]
}
```

### history

```bash
modem-cat history [connection-id]
```

**Output**:
```json
{
  "success": true,
  "commands": [
    { "command": "AT+CSQ", "status": "OK", "sentAt": "2026-04-04T10:00:00Z" },
    { "command": "ATI", "status": "OK", "sentAt": "2026-04-04T10:01:00Z" }
  ]
}
```

## Configuration Commands

### config set

```bash
modem-cat config set <key> <value> [connection-id]
```

**Example**:
```bash
modem-cat config set network-mode 5G conn_abc123
```

### config get

```bash
modem-cat config get [key] [connection-id]
```

**Output**:
```json
{
  "success": true,
  "config": {
    "networkMode": "5G",
    "apn": "cmnet",
    "pin": "****"
  }
}
```

## Global Options

- `--json`: Output in JSON format (default for non-interactive use)
- `--human`: Output in human-readable format (default for terminal)
- `--output <file>`: Write output to file instead of stdout
- `--verbose`: Enable verbose logging
- `--help`: Show help message
- `--version`: Show version information
