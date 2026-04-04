/**
 * Connect Command
 * Connect to a modem via USB Serial, Ethernet, or TTL
 */

import { connectionManager } from '../../core/connections/connection-manager.js';
import { formatJson, formatHuman } from '../../lib/formatter.js';
import type { OutputFormat } from '../../core/types/index.js';

export const connectCommand = {
  name: 'connect',
  description: 'Connect to a modem via USB Serial, Ethernet, or TTL',
  options: {
    type: {
      type: 'string',
      short: 't',
    },
    port: {
      type: 'string',
      short: 'p',
    },
    baud: {
      type: 'string',
      short: 'b',
    },
    host: {
      type: 'string',
      short: 'h',
    },
    timeout: {
      type: 'string',
    },
  },
  run: async (args: Record<string, unknown>, globalOpts: { json: boolean; human: boolean }) => {
    const type = args.type as string;
    if (!type) {
      throw new Error('--type is required (usb, ethernet, or ttl)');
    }

    const format: OutputFormat = globalOpts.json ? 'json' : 'human';
    let connection;

    if (type === 'usb' || type === 'ttl') {
      const port = args.port as string;
      if (!port) {
        throw new Error('--port is required for USB/TTL connection');
      }

      const baudRate = parseInt(args.baud as string || '115200', 10);
      connection = connectionManager.createConnection(
        type === 'usb' ? 'USB_SERIAL' : 'TTL',
        { port, baudRate }
      );
    } else if (type === 'ethernet') {
      const host = args.host as string;
      const port = args.port as string;
      if (!host || !port) {
        throw new Error('--host and --port are required for Ethernet connection');
      }

      connection = connectionManager.createConnection('ETHERNET', {
        host,
        port: parseInt(port, 10),
        protocol: 'TCP',
      });
    } else {
      throw new Error('Invalid connection type. Use: usb, ethernet, or ttl');
    }

    await connectionManager.connect();

    const output = {
      success: true,
      connection: {
        id: connection.id,
        type: connection.type,
        status: connection.status,
      },
    };

    console.log(format === 'json' ? formatJson(output) : formatHuman(output));
  },
};
