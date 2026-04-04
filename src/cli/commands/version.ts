/**
 * Version Command
 * Display version information
 */

import { formatJson, formatHuman } from '../../lib/formatter.js';
import type { OutputFormat } from '../../core/types/index.js';

const VERSION = '0.1.0';

export const versionCommand = {
  name: 'version',
  description: 'Display version information',
  options: {},
  run: async (args: Record<string, unknown>, globalOpts: { json: boolean; human: boolean }) => {
    const format: OutputFormat = globalOpts.json ? 'json' : 'human';

    const output = {
      name: 'modem-cat',
      version: VERSION,
      description: '5G Modem调试工具',
    };

    console.log(format === 'json' ? formatJson(output) : formatHuman(output));
  },
};
