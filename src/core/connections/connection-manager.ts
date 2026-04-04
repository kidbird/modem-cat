/**
 * Connection Manager - State machine for managing connections
 */

import type { Connection, ConnectionStatus, ConnectionType, ConnectionParams } from '../types/index.js';
import { logger } from '../../lib/logger.js';

export class ConnectionManager {
  private connection: Connection | null = null;

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
   * Connect to modem
   */
  async connect(): Promise<Connection> {
    if (!this.connection) {
      throw new Error('No connection created');
    }

    this.setStatus('CONNECTING');

    // In real implementation, this would establish the actual connection
    // For now, we just mark as connected
    this.setStatus('CONNECTED');

    return this.connection;
  }

  /**
   * Disconnect from modem
   */
  async disconnect(): Promise<void> {
    if (!this.connection) {
      throw new Error('No active connection');
    }

    this.setStatus('DISCONNECTED');
    logger.info('Disconnected', { id: this.connection.id });
  }

  /**
   * Close and cleanup connection
   */
  close(): void {
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
