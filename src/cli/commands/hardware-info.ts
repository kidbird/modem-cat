/**
 * Hardware Info Command
 * Get hardware information from modem
 */

import { connectionManager } from '../../core/connections/connection-manager.js';
import { formatJson, formatHuman } from '../../lib/formatter.js';
import type { OutputFormat } from '../../core/types/index.js';

export const hardwareInfoCommand = {
  name: 'hardware-info',
  description: 'Get hardware information from modem',
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

    // Mock hardware info - in real implementation, would query modem
    const output = {
      success: true,
      hardware: {
        model: 'MF269',
        firmwareVersion: 'V3.2.1',
        hardwareVersion: 'Rev.3',
        manufacturer: 'Fibocom',
        cpuUsage: 25,
        memoryUsage: 45,
        uptime: 86400,
      },
    };

    console.log(format === 'json' ? formatJson(output) : formatHuman(output));
  },
};
