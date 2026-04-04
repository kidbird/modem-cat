/**
 * Network Info Command
 * Get detailed cellular network information
 */

import { connectionManager } from '../../core/connections/connection-manager.js';
import { formatJson, formatHuman } from '../../lib/formatter.js';
import type { OutputFormat } from '../../core/types/index.js';

export const networkInfoCommand = {
  name: 'network-info',
  description: 'Get detailed cellular network information',
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

    // Mock network info - in real implementation, would query modem
    const output = {
      success: true,
      network: {
        operatorName: 'China Mobile',
        plmn: '46000',
        band: 'n78',
        bandwidth: 100,
        pci: 123,
        rsrp: -85,
        rsrq: -12,
        snr: 15,
        neighborCells: [],
      },
    };

    console.log(format === 'json' ? formatJson(output) : formatHuman(output));
  },
};
