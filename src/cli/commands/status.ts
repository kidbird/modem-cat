/**
 * Status Command
 * Get modem status information
 */

import { connectionManager } from '../../core/connections/connection-manager.js';
import { formatJson, formatHuman } from '../../lib/formatter.js';
import type { OutputFormat } from '../../core/types/index.js';

export const statusCommand = {
  name: 'status',
  description: 'Get modem status information',
  options: {
    connection: {
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

    // Mock status - in real implementation, would query modem
    const output = {
      success: true,
      status: {
        registration: 'REGISTERED',
        signalStrength: {
          rsrp: -85,
          rsrq: -12,
        },
        connectionMode: '5G',
        imei: '123456789012345',
        operator: 'China Mobile',
      },
    };

    console.log(format === 'json' ? formatJson(output) : formatHuman(output));
  },
};
