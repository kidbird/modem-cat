/**
 * Connection Manager - State machine for managing connections
 */

import { SerialPort } from 'serialport';
import type { Connection, ConnectionStatus, ConnectionType, ConnectionParams } from '../types/index.js';
import { logger } from '../../lib/logger.js';

export class ConnectionManager {
  private connection: Connection | null = null;
  private serialPort: SerialPort | null = null;

  /**
   * Create a new connection
   */
  createConnection(type: ConnectionType, params: ConnectionParams): Connection {
    const id = this.generateId(type, params);
    const now = new Date();

    this.connection = {
      id,
      type,
      params,
      status: 'DISCONNECTED',
      createdAt: now,
      lastActivity: now,
    };

    logger.info('Created connection', { id, type });
    return this.connection;
  }

  /**
   * Get current connection
   */
  getConnection(): Connection | null {
    return this.connection;
  }

  /**
   * Get connection status
   */
  getStatus(): ConnectionStatus {
    return this.connection?.status || 'DISCONNECTED';
  }

  /**
   * Get serial port instance
   */
  getSerialPort(): SerialPort | null {
    return this.serialPort;
  }

  /**
   * Update connection status
   */
  setStatus(status: ConnectionStatus): void {
    if (this.connection) {
      this.connection.status = status;
      this.connection.lastActivity = new Date();
      logger.info('Connection status changed', { id: this.connection.id, status });
    }
  }

  /**
   * Connect to modem via USB Serial
   */
  async connect(): Promise<Connection> {
    if (!this.connection) {
      throw new Error('No connection created');
    }

    if (this.connection.type !== 'USB_SERIAL' && this.connection.type !== 'TTL') {
      throw new Error('Unsupported connection type for serial connection');
    }

    const { port, baudRate = 115200 } = this.connection.params;
    if (!port) {
      throw new Error('Port is required');
    }

    this.setStatus('CONNECTING');

    return new Promise((resolve, reject) => {
      // Close existing port if any
      if (this.serialPort?.isOpen) {
        this.serialPort.close();
        this.serialPort = null;
      }

      this.serialPort = new SerialPort({
        path: port,
        baudRate,
        dataBits: 8,
        parity: 'none',
        stopBits: 1,
        autoOpen: false,
      });

      this.serialPort.on('error', (err) => {
        logger.error('Serial port error', { error: err.message });
        this.setStatus('ERROR');
        reject(err);
      });

      this.serialPort.on('close', () => {
        logger.info('Serial port closed');
        this.setStatus('DISCONNECTED');
      });

      this.serialPort.open((err) => {
        if (err) {
          logger.error('Failed to open serial port', { error: err.message });
          this.setStatus('ERROR');
          reject(err);
          return;
        }

        logger.info('Serial port opened', { port, baudRate });

        // Wait for modem to initialize (typically 3 seconds)
        setTimeout(() => {
          // Flush input buffer to clear any stale data
          if (this.serialPort?.isOpen) {
            this.serialPort.flush(() => {
              logger.info('Serial buffer flushed');
              this.setStatus('CONNECTED');
              resolve(this.connection!);
            });
          } else {
            this.setStatus('CONNECTED');
            resolve(this.connection!);
          }
        }, 3000);
      });
    });
  }

  /**
   * Disconnect from modem
   */
  async disconnect(): Promise<void> {
    if (this.serialPort?.isOpen) {
      this.serialPort.close((err) => {
        if (err) {
          logger.error('Error closing serial port', { error: err.message });
        }
      });
    }
    this.setStatus('DISCONNECTED');
    logger.info('Disconnected', { id: this.connection?.id });
  }

  /**
   * Close and cleanup connection
   */
  close(): void {
    if (this.serialPort?.isOpen) {
      this.serialPort.close();
    }
    this.serialPort = null;
    if (this.connection) {
      logger.info('Closing connection', { id: this.connection.id });
      this.connection = null;
    }
  }

  /**
   * Generate unique connection ID
   */
  private generateId(type: ConnectionType, params: ConnectionParams): string {
    const prefix = type === 'ETHERNET'
      ? `eth_${params.host}:${params.port}`
      : type === 'USB_SERIAL'
        ? `usb_${params.port}`
        : `ttl_${params.port}`;

    const timestamp = Date.now().toString(36);
    return `${prefix}_${timestamp}`;
  }
}

// Singleton instance
export const connectionManager = new ConnectionManager();