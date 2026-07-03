# Data Model: 5G Modem调试工具

**Date**: 2026-04-04
**Feature**: 001-modem-debug-tool

## Entities

### Connection

| Field | Type | Description |
|-------|------|-------------|
| id | string | Unique connection identifier |
| type | ConnectionType | USB_SERIAL, ETHERNET, TTL |
| params | ConnectionParams | Type-specific parameters |
| status | ConnectionStatus | CONNECTED, DISCONNECTED, ERROR |
| createdAt | Date | Connection start time |
| lastActivity | Date | Last activity timestamp |

**ConnectionParams** (union type):
- USB_SERIAL: { port: string, baudRate: number, dataBits: 8, stopBits: 1, parity: 'none' }
- ETHERNET: { host: string, port: number, protocol: 'TCP' | 'UDP' }
- TTL: { port: string, baudRate: number }

### ModemStatus

| Field | Type | Description |
|-------|------|-------------|
| registration | NetworkRegistrationState | NOT_REGISTERED, REGISTERED, SEARCHING, DENIED |
| signalStrength | SignalStrength | RSRP, RSRQ in dBm |
| connectionMode | ConnectionMode | 4G, 5G, NSA, SA |
| imei | string | International Mobile Equipment Identity |
| imsi | string | Subscriber Identity (if available) |
| operator | string | Operator name |

### NetworkInfo

| Field | Type | Description |
|-------|------|-------------|
| operatorName | string | Operator display name |
| plmn | string | PLMN code (MCC+MNC) |
| band | string | Frequency band (e.g., n78, B1) |
| bandwidth | number | Channel bandwidth in MHz |
| pci | number | Physical Cell ID |
| rsrp | number | Reference Signal Received Power (dBm) |
| rsrq | number | Reference Signal Received Quality (dB) |
| snr | number | Signal to Noise Ratio (dB) |
| neighborCells | NeighborCell[] | Available neighbor cells |

### HardwareInfo

| Field | Type | Description |
|-------|------|-------------|
| model | string | Modem model name |
| firmwareVersion | string | Firmware version |
| hardwareVersion | string | Hardware revision |
| manufacturer | string | Manufacturer name |
| cpuUsage | number | CPU usage percentage |
| memoryUsage | number | Memory usage percentage |
| uptime | number | Running time in seconds |
| temperature | number | Temperature (if available) |

### ATCommand

| Field | Type | Description |
|-------|------|-------------|
| id | string | Unique command identifier |
| command | string | AT command string |
| response | string | Command response |
| status | ATCommandStatus | PENDING, SENT, OK, ERROR, TIMEOUT |
| sentAt | Date | Command sent timestamp |
| receivedAt | Date | Response received timestamp |
| duration | number | Execution time in milliseconds |

### CommandHistory

| Field | Type | Description |
|-------|------|-------------|
| id | string | History entry ID |
| connectionId | string | Associated connection |
| commands | ATCommand[] | List of executed commands |
| maxSize | number | Maximum history entries (default 100) |

## Relationships

```
Connection 1───* ATCommand
Connection 1───1 ModemStatus (current)
Connection 1───1 NetworkInfo
Connection 1───1 HardwareInfo
```

## State Machines

### Connection State

```
DISCONNECTED ──connect()──> CONNECTING ──success──> CONNECTED
CONNECTED ──disconnect()──> DISCONNECTED
CONNECTED ──error──> ERROR
ERROR ──reconnect()──> CONNECTING
```

### AT Command State

```
PENDING ──send──> SENT ──response──> OK
                         └──error──> ERROR
SENT ──timeout──> TIMEOUT
```

## Validation Rules

- Connection type must be one of: USB_SERIAL, ETHERNET, TTL
- Baud rate for serial must be standard: 9600, 19200, 38400, 57600, 115200, 230400, 460800, 921600
- IP address must be valid IPv4 format
- Port must be 1-65535
- AT command must not exceed 1000 characters
