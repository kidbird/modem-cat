/**
 * List Ports Command
 * List available serial ports
 */

import { SerialPort } from 'serialport';
import { formatJson, formatHuman } from '../../lib/formatter.js';
import type { OutputFormat } from '../../core/types/index.js';

export const listPortsCommand = {
  name: 'list-ports',
  description: 'List available serial ports',
  options: {},
  run: async (args: Record<string, unknown>, globalOpts: { json: boolean; human: boolean }) => {
    const format: OutputFormat = globalOpts.json ? 'json' : 'human';

    try {
      const ports = await SerialPort.list();

      const output = {
        success: true,
        ports: ports.map(p => ({
          path: p.path,
          description: p.serialNumber || p.manufacturer || null,
          manufacturer: p.manufacturer || null,
        })),
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
