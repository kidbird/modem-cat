/**
 * AT Command Executor
 * Execute AT commands on modem
 */

import { SerialPort } from 'serialport';
import { connectionManager } from '../connections/connection-manager.js';
import type { ATCommand, ATCommandStatus } from '../types/index.js';

export class ATExecutor {
  private pendingResponse: string = '';
  private responseCallback: ((data: string) => void) | null = null;

  /**
   * Execute AT command
   */
  async execute(command: string, timeout: number = 5000): Promise<ATCommand> {
    const connection = connectionManager.getConnection();
    if (!connection || connection.status !== 'CONNECTED') {
      throw new Error('Not connected');
    }

    const serialPort = connectionManager.getSerialPort();
    if (!serialPort || !serialPort.isOpen) {
      throw new Error('Serial port not open');
    }

    const id = `cmd_${Date.now()}`;
    const sentAt = new Date();

    return new Promise((resolve, reject) => {
      let response = '';
      let timeoutId: NodeJS.Timeout;
      let received = false;

      const cleanup = () => {
        clearTimeout(timeoutId);
        serialPort.removeListener('data', onData);
      };

      const onData = (chunk: Buffer) => {
        const data = chunk.toString();
        response += data;

        // Check for command echo (the command we sent is echoed back)
        const lines = response.split('\r\n');
        for (const line of lines) {
          if (line === command || line === command.trim()) {
            // Command was echoed, continue waiting for response
            continue;
          }
          if (line === 'OK' || line === 'ERROR' || line.startsWith('+CME ERROR') || line.startsWith('+CMS ERROR')) {
            received = true;
            cleanup();
            const receivedAt = new Date();
            const duration = receivedAt.getTime() - sentAt.getTime();

            let status: ATCommandStatus = 'OK';
            if (line === 'ERROR' || line.startsWith('+CME ERROR') || line.startsWith('+CMS ERROR')) {
              status = 'ERROR';
            }

            resolve({
              id,
              command,
              response: response.trim(),
              status,
              sentAt,
              receivedAt,
              duration,
            });
            return;
          }
        }
      };

      serialPort.on('data', onData);

      // Set timeout
      timeoutId = setTimeout(() => {
        cleanup();
        const receivedAt = new Date();
        const duration = receivedAt.getTime() - sentAt.getTime();
        resolve({
          id,
          command,
          response: response.trim() || 'TIMEOUT',
          status: 'TIMEOUT',
          sentAt,
          receivedAt,
          duration,
        });
      }, timeout);

      // Send command with \r\n terminator
      const cmdBuffer = Buffer.from(command + '\r\n');
      serialPort.write(cmdBuffer, (err) => {
        if (err) {
          cleanup();
          reject(err);
        }
      });
    });
  }

  /**
   * Execute multiple AT commands
   */
  async executeBatch(commands: string[], timeout: number = 5000): Promise<ATCommand[]> {
    const results: ATCommand[] = [];

    for (const cmd of commands) {
      const result = await this.execute(cmd, timeout);
      results.push(result);

      // Small delay between commands
      await new Promise(resolve => setTimeout(resolve, 100));
    }

    return results;
  }
}

export const atExecutor = new ATExecutor();