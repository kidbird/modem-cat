/**
 * Disconnect Command
 * Disconnect from connected modem
 */

import { connectionManager } from '../../core/connections/connection-manager.js';
import { formatJson, formatHuman } from '../../lib/formatter.js';
import type { OutputFormat } from '../../core/types/index.js';

export const disconnectCommand = {
  name: 'disconnect',
  description: 'Disconnect from connected modem',
  options: {},
  run: async (args: Record<string, unknown>, globalOpts: { json: boolean; human: boolean }) => {
    const format: OutputFormat = globalOpts.json ? 'json' : 'human';

    const connection = connectionManager.getConnection();
    if (!connection) {
      const output = { success: false, error: 'No active connection' };
      console.log(format === 'json' ? formatJson(output) : formatHuman(output));
      return;
    }

    await connectionManager.disconnect();

    const output = {
      success: true,
      message: `Disconnected from ${connection.id}`,
    };

    console.log(format === 'json' ? formatJson(output) : formatHuman(output));
  },
};
