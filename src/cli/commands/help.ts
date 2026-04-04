/**
 * Help Command
 * Display help information
 */

import { formatJson, formatHuman } from '../../lib/formatter.js';
import type { OutputFormat } from '../../core/types/index.js';

const commands = [
  { name: 'connect', description: 'Connect to modem (usb/ethernet/ttl)', usage: 'modem-cat connect --type <usb|ethernet|ttl> [options]' },
  { name: 'disconnect', description: 'Disconnect from modem', usage: 'modem-cat disconnect' },
  { name: 'list-ports', description: 'List available serial ports', usage: 'modem-cat list-ports' },
  { name: 'status', description: 'Get modem status', usage: 'modem-cat status [--connection <id>]' },
  { name: 'at', description: 'Send AT command', usage: 'modem-cat at --command <AT command>' },
  { name: 'network-info', description: 'Get network information', usage: 'modem-cat network-info' },
  { name: 'hardware-info', description: 'Get hardware information', usage: 'modem-cat hardware-info' },
  { name: 'config', description: 'Get/set modem config', usage: 'modem-cat config [get|set] --key <key> --value <value>' },
  { name: 'history', description: 'View command history', usage: 'modem-cat history [--clear]' },
  { name: 'help', description: 'Show this help', usage: 'modem-cat help' },
  { name: 'version', description: 'Show version', usage: 'modem-cat --version' },
];

export const helpCommand = {
  name: 'help',
  description: 'Display help information',
  options: {},
  run: async (args: Record<string, unknown>, globalOpts: { json: boolean; human: boolean }) => {
    const format: OutputFormat = globalOpts.json ? 'json' : 'human';

    const output = {
      name: 'modem-cat',
      version: '0.1.0',
      description: '5G Modem调试工具 - Cross-platform CLI and desktop tool for debugging 5G modems',
      usage: 'modem-cat <command> [options]',
      globalOptions: {
        '--json': 'Output in JSON format',
        '--human': 'Output in human-readable format',
        '--verbose': 'Enable verbose logging',
        '--help': 'Show help',
        '--version': 'Show version',
      },
      commands: commands.map(c => ({ ...c, usage: c.usage.replace('modem-cat ', '') })),
    };

    console.log(format === 'json' ? formatJson(output) : formatHuman(output));
  },
};
