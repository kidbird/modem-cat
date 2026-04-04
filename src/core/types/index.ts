// Connection Types
export type ConnectionType = 'USB_SERIAL' | 'ETHERNET' | 'TTL';
export type ConnectionStatus = 'CONNECTED' | 'DISCONNECTED' | 'CONNECTING' | 'ERROR';

export interface ConnectionParams {
  // USB Serial / TTL
  port?: string;
  baudRate?: number;
  // Ethernet
  host?: string;
  port?: number;
  protocol?: 'TCP' | 'UDP';
}

export interface Connection {
  id: string;
  type: ConnectionType;
  params: ConnectionParams;
  status: ConnectionStatus;
  createdAt: Date;
  lastActivity: Date;
}

// Network Types
export type NetworkRegistrationState = 'NOT_REGISTERED' | 'REGISTERED' | 'SEARCHING' | 'DENIED';
export type ConnectionMode = '4G' | '5G' | 'NSA' | 'SA';

export interface SignalStrength {
  rsrp: number;  // dBm
  rsrq: number; // dB
}

export interface ModemStatus {
  registration: NetworkRegistrationState;
  signalStrength: SignalStrength;
  connectionMode: ConnectionMode;
  imei: string;
  imsi?: string;
  operator?: string;
}

export interface NetworkInfo {
  operatorName: string;
  plmn: string;
  band: string;
  bandwidth: number;
  pci: number;
  rsrp: number;
  rsrq: number;
  snr: number;
  neighborCells: NeighborCell[];
}

export interface NeighborCell {
  pci: number;
  rsrp: number;
  rsrq: number;
}

// Hardware Types
export interface HardwareInfo {
  model: string;
  firmwareVersion: string;
  hardwareVersion: string;
  manufacturer: string;
  cpuUsage?: number;
  memoryUsage?: number;
  uptime?: number;
  temperature?: number;
}

// AT Command Types
export type ATCommandStatus = 'PENDING' | 'SENT' | 'OK' | 'ERROR' | 'TIMEOUT';

export interface ATCommand {
  id: string;
  command: string;
  response: string;
  status: ATCommandStatus;
  sentAt: Date;
  receivedAt?: Date;
  duration: number;
}

export interface CommandHistory {
  id: string;
  connectionId: string;
  commands: ATCommand[];
  maxSize: number;
}

// CLI Types
export type OutputFormat = 'json' | 'human';

export interface CLIOptions {
  json: boolean;
  human: boolean;
  output?: string;
  verbose: boolean;
  help: boolean;
  version: boolean;
}
