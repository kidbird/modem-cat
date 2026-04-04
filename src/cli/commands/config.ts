/**
 * Config Command
 * Get or set modem configuration
 */

import { connectionManager } from '../../core/connections/connection-manager.js';
import { formatJson, formatHuman } from '../../lib/formatter.js';
import type { OutputFormat } from '../../core/types/index.js';

export const configCommand = {
  name: 'config',
  description: 'Get or set modem configuration',
  options: {
    action: {
      type: 'string',
      short: 'a',
    },
    key: {
      type: 'string',
      short: 'k',
    },
    value: {
      type: 'string',
      short: 'v',
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

    const action = (args.action || 'get') as string;
    const key = args.key as string;
    const value = args.value as string | undefined;

    if (action === 'set') {
      if (!key || !value) {
        throw new Error('--key and --value are required for set action');
      }

      // Mock config set - in real implementation, would send to modem
      const output = {
        success: true,
        message: `Config ${key} set to ${value}`,
      };
      console.log(format === 'json' ? formatJson(output) : formatHuman(output));
    } else {
      // Mock config get
      const output = {
        success: true,
        config: {
          networkMode: '5G',
          apn: 'cmnet',
          pin: '****',
        },
      };
      console.log(format === 'json' ? formatJson(output) : formatHuman(output));
    }
  },
};
