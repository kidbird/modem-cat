/**
 * AT Command
 * Send AT command to modem
 */

import { connectionManager } from '../../core/connections/connection-manager.js';
import { atExecutor } from '../../core/modem/at-executor.js';
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
    timeout: {
      type: 'string',
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

    const timeout = parseInt(args.timeout as string || '5000', 10);

    try {
      const result = await atExecutor.execute(command, timeout);

      const output = {
        success: result.status === 'OK',
        command: {
          id: result.id,
          command: result.command,
          response: result.response,
          status: result.status,
          duration: result.duration,
        },
      };

      console.log(format === 'json' ? formatJson(output) : formatHuman(output));
    } catch (error) {
      const output = {
        success: false,
        error: error instanceof Error ? error.message : String(error),
      };
      console.log(format === 'json' ? formatJson(output) : formatHuman(output));
    }
  },
};