/**
 * List Ports Command
 * List available serial ports
 */

import { formatJson, formatHuman } from '../../lib/formatter.js';
import type { OutputFormat } from '../../core/types/index.js';

export const listPortsCommand = {
  name: 'list-ports',
  description: 'List available serial ports',
  options: {},
  run: async (args: Record<string, unknown>, globalOpts: { json: boolean; human: boolean }) => {
    const format: OutputFormat = globalOpts.json ? 'json' : 'human';

    // Mock ports for now - in real implementation, this would call Tauri backend
    const ports = [
      { path: '/dev/cu.usbserial-1420', description: 'USB Serial Device', manufacturer: 'Silicon Labs' },
      { path: '/dev/cu.Bluetooth-Incoming-Port', description: 'Bluetooth', manufacturer: null },
    ];

    const output = {
      success: true,
      ports,
    };

    console.log(format === 'json' ? formatJson(output) : formatHuman(output));
  },
};
