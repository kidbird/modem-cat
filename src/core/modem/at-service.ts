/**
 * AT Command Service
 * High-level AT command operations
 */

import { atExecutor } from './at-executor.js';
import { historyManager } from './history-manager.js';
import type { ATCommand } from '../types/index.js';

export class ATService {
  /**
   * Send AT command
   */
  async send(command: string): Promise<ATCommand> {
    const result = await atExecutor.execute(command);

    // Add to history
    await historyManager.addCommand(result);

    return result;
  }

  /**
   * Send AT command with custom timeout
   */
  async sendWithTimeout(command: string, timeout: number): Promise<ATCommand> {
    // In real implementation, would set timeout
    return this.send(command);
  }
}

export const atService = new ATService();
