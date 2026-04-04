#!/usr/bin/env bun
/**
 * Modem Cat CLI Entry Point
 * Cross-platform CLI tool for debugging 5G modems
 */

import { parseArgs } from 'util';
import { logger } from '../lib/logger.js';
import { connectionManager } from '../core/connections/connection-manager.js';
import { formatJson, formatHuman } from '../lib/formatter.js';
import type { OutputFormat } from '../core/types/index.js';

// Import commands
import { connectCommand } from './commands/connect.js';
import { disconnectCommand } from './commands/disconnect.js';
import { listPortsCommand } from './commands/list-ports.js';
import { statusCommand } from './commands/status.js';
import { atCommand } from './commands/at.js';
import { networkInfoCommand } from './commands/network-info.js';
import { hardwareInfoCommand } from './commands/hardware-info.js';
import { configCommand } from './commands/config.js';
import { historyCommand } from './commands/history.js';
import { helpCommand } from './commands/help.js';
import { versionCommand } from './commands/version.js';

interface GlobalOptions {
  json: boolean;
  human: boolean;
  verbose: boolean;
  help: boolean;
  version: boolean;
}

interface Command {
  name: string;
  description: string;
  run: (args: Record<string, unknown>, globalOpts: GlobalOptions) => Promise<void>;
}

const commands: Command[] = [
  connectCommand,
  disconnectCommand,
  listPortsCommand,
  statusCommand,
  atCommand,
  networkInfoCommand,
  hardwareInfoCommand,
  configCommand,
  historyCommand,
  helpCommand,
  versionCommand,
];

async function main() {
  const args = process.argv.slice(2);

  // Handle --help and --version first
  if (args.includes('--help') || args.includes('-h')) {
    await helpCommand.run({}, { json: false, human: false, verbose: false, help: false, version: false });
    process.exit(0);
  }

  if (args.includes('--version') || args.includes('-v')) {
    await versionCommand.run({}, { json: false, human: false, verbose: false, help: false, version: false });
    process.exit(0);
  }

  // Parse command name
  const commandName = args[0] || 'help';
  const commandArgs = args.slice(1);

  // Find command
  const command = commands.find(c => c.name === commandName);

  if (!command) {
    console.error(`Unknown command: ${commandName}`);
    console.error('Run "modem-cat --help" for usage information');
    process.exit(1);
  }

  // Parse global options
  const globalOpts: GlobalOptions = {
    json: args.includes('--json'),
    human: args.includes('--human'),
    verbose: args.includes('--verbose'),
    help: false,
    version: false,
  };

  // Set log level based on verbose flag
  if (globalOpts.verbose) {
    process.env.LOG_LEVEL = '0';
  }

  // Run command
  try {
    // Parse command-specific args
    const parsed = parseArgs({
      args: commandArgs,
      options: command.options || {},
      strict: false,
    });

    await command.run(parsed.values as Record<string, unknown>, globalOpts);
  } catch (error) {
    logger.error('Command failed', error);

    if (globalOpts.json) {
      console.log(formatJson({
        success: false,
        error: error instanceof Error ? error.message : String(error),
      }));
    } else {
      console.error(`Error: ${error instanceof Error ? error.message : String(error)}`);
    }

    process.exit(1);
  }
}

main().catch(error => {
  logger.error('Fatal error', error);
  console.error(`Fatal error: ${error instanceof Error ? error.message : String(error)}`);
  process.exit(1);
});
