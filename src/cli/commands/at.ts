/**
 * AT Command
 * Send AT command to modem
 */

import { connectionManager } from '../../core/connections/connection-manager.js';
import { formatJson, formatHuman } from '../../lib/formatter.js';
import type { OutputFormat } from '../../core/types/index.js';

export const atCommand = {
  name: 'at',
  description: 'Send AT command to modem',
  options: {
    command: {
      type: 'string',
      short: 'c',
    },
  },
  run: async (args: Record<string, unknown>, globalOpts: { json: boolean; human: boolean }) => {
    const format: OutputFormat = globalOpts.json ? 'json' : 'human';

    const connection = connectionManager.getConnection();
    if (!connection || connection.status !== 'CONNECTED') {
      const output = { success: false, error: 'Not connected' };
      console.log(format === 'json' ? formatJson(output) : formatHuman(output));
      return;
    }

    const command = args.command as string;
    if (!command) {
      throw new Error('--command is required');
    }

    // Mock response - in real implementation, would send to modem
    const response = `${command}\r\nOK\r\n`;

    const output = {
      success: true,
      command: {
        id: `cmd_${Date.now()}`,
        command,
        response,
        status: 'OK',
        duration: 45,
      },
    };

    console.log(format === 'json' ? formatJson(output) : formatHuman(output));
  },
};
