/**
 * AT Command Executor
 * Execute AT commands on modem
 */

import { connectionManager } from '../connections/connection-manager.js';
import { atParser, type ParsedATResponse } from '../parser/at-parser.js';
import type { ATCommand, ATCommandStatus } from '../types/index.js';

export class ATExecutor {
  /**
   * Execute AT command
   */
  async execute(command: string): Promise<ATCommand> {
    const connection = connectionManager.getConnection();
    if (!connection || connection.status !== 'CONNECTED') {
      throw new Error('Not connected');
    }

    const id = `cmd_${Date.now()}`;
    const sentAt = new Date();

    // In real implementation, would send to actual modem
    // For now, return mock response
    const response = `${command}\r\nOK\r\n`;
    const receivedAt = new Date();

    const duration = receivedAt.getTime() - sentAt.getTime();

    return {
      id,
      command,
      response,
      status: 'OK',
      sentAt,
      receivedAt,
      duration,
    };
  }

  /**
   * Execute multiple AT commands
   */
  async executeBatch(commands: string[]): Promise<ATCommand[]> {
    const results: ATCommand[] = [];

    for (const cmd of commands) {
      const result = await this.execute(cmd);
      results.push(result);

      // Small delay between commands
      await new Promise(resolve => setTimeout(resolve, 50));
    }

    return results;
  }
}

export const atExecutor = new ATExecutor();
