/**
 * Logger module for Modem Cat
 * Provides structured logging with different levels
 */

export enum LogLevel {
  DEBUG = 0,
  INFO = 1,
  WARN = 2,
  ERROR = 3,
}

const LOG_LEVEL = parseInt(process.env.LOG_LEVEL || '1', 10);

function formatMessage(level: string, message: string, ...args: unknown[]): string {
  const timestamp = new Date().toISOString();
  const formattedArgs = args.length > 0 ? ' ' + args.map(a => JSON.stringify(a)).join(' ') : '';
  return `[${timestamp}] [${level}] ${message}${formattedArgs}`;
}

export const logger = {
  debug(message: string, ...args: unknown[]): void {
    if (LOG_LEVEL <= LogLevel.DEBUG) {
      console.debug(formatMessage('DEBUG', message, ...args));
    }
  },

  info(message: string, ...args: unknown[]): void {
    if (LOG_LEVEL <= LogLevel.INFO) {
      console.log(formatMessage('INFO', message, ...args));
    }
  },

  warn(message: string, ...args: unknown[]): void {
    if (LOG_LEVEL <= LogLevel.WARN) {
      console.warn(formatMessage('WARN', message, ...args));
    }
  },

  error(message: string, ...args: unknown[]): void {
    if (LOG_LEVEL <= LogLevel.ERROR) {
      console.error(formatMessage('ERROR', message, ...args));
    }
  },
};

export default logger;
