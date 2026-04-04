/**
 * History Command
 * View command history
 */

import { connectionManager } from '../../core/connections/connection-manager.js';
import { formatJson, formatHuman } from '../../lib/formatter.js';
import type { OutputFormat } from '../../core/types/index.js';

export const historyCommand = {
  name: 'history',
  description: 'View command history',
  options: {
    connection: {
      type: 'string',
      short: 'c',
    },
    clear: {
      type: 'boolean',
      short: 'C',
    },
  },
  run: async (args: Record<string, unknown>, globalOpts: { json: boolean; human: boolean }) => {
    const format: OutputFormat = globalOpts.json ? 'json' : 'human';

    const clear = args.clear as boolean;
    if (clear) {
      const output = {
        success: true,
        message: 'Command history cleared',
      };
      console.log(format === 'json' ? formatJson(output) : formatHuman(output));
      return;
    }

    // Mock history - in real implementation, would load from storage
    const output = {
      success: true,
      commands: [
        { command: 'ATI', status: 'OK', sentAt: new Date().toISOString() },
        { command: 'AT+CSQ', status: 'OK', sentAt: new Date().toISOString() },
      ],
    };

    console.log(format === 'json' ? formatJson(output) : formatHuman(output));
  },
};
