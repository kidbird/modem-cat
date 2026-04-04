/**
 * Command History Manager
 * Manage AT command history
 */

import type { ATCommand, CommandHistory } from '../types/index.js';

const MAX_HISTORY = 100;

export class HistoryManager implements CommandHistory {
  id = 'default';
  connectionId = 'default';
  commands: ATCommand[] = [];
  maxSize = MAX_HISTORY;

  /**
   * Add command to history
   */
  async addCommand(command: ATCommand): Promise<void> {
    this.commands.unshift(command);

    // Trim if needed
    if (this.commands.length > this.maxSize) {
      this.commands = this.commands.slice(0, this.maxSize);
    }
  }

  /**
   * Get command history
   */
  getHistory(): ATCommand[] {
    return this.commands;
  }

  /**
   * Clear history
   */
  clear(): void {
    this.commands = [];
  }

  /**
   * Get history size
   */
  size(): number {
    return this.commands.length;
  }
}

export const historyManager = new HistoryManager();
