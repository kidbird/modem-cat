/**
 * AT Command Response Parser
 * Parses AT command responses from modems
 */

import type { ATCommandStatus } from '../types/index.js';

export interface ParsedATResponse {
  status: ATCommandStatus;
  lines: string[];
  raw: string;
}

export class ATParser {
  /**
   * Parse AT command response
   */
  parse(response: string): ParsedATResponse {
    const lines = response.split(/\r?\n/).map(line => line.trim()).filter(line => line.length > 0);

    // Determine status from response
    let status: ATCommandStatus = 'OK';

    if (lines.length === 0) {
      status = 'ERROR';
    } else {
      const lastLine = lines[lines.length - 1].toUpperCase();

      if (lastLine.includes('OK')) {
        status = 'OK';
      } else if (lastLine.includes('ERROR') || lastLine.includes('FAIL')) {
        status = 'ERROR';
      } else if (lastLine.includes('TIMEOUT') || lastLine.includes('NO CARRIER')) {
        status = 'TIMEOUT';
      }
    }

    return {
      status,
      lines,
      raw: response,
    };
  }

  /**
   * Extract value from response by pattern
   */
  extractValue(response: string, pattern: string): string | null {
    const match = response.match(new RegExp(pattern, 'i'));
    return match ? match[1] || match[0] : null;
  }

  /**
   * Parse +CSQ response (signal quality)
   */
  parseCSQ(response: string): { rssi: number; ber: number } | null {
    const match = response.match(/\+CSQ:\s*(\d+),(\d+)/);
    if (!match) return null;

    // RSSI: 0-31 (99 = not detectable), BER: 0-7 (99 = not detectable)
    const rssi = parseInt(match[1], 10);
    const ber = parseInt(match[2], 10);

    return { rssi, ber };
  }

  /**
   * Parse +CEREG response (network registration)
   */
  parseCEREG(response: string): { n: number; stat: number } | null {
    const match = response.match(/\+CEREG:\s*(\d+),(\d+)/);
    if (!match) return null;

    return {
      n: parseInt(match[1], 10),
      stat: parseInt(match[2], 10),
    };
  }

  /**
   * Parse +CPSI response (system info)
   */
  parseCPSI(response: string): {
    system: string;
    operation: string;
    bandwidth: number;
    rsrp: number;
    rsrq: number;
    snr: number;
  } | null {
    const match = response.match(/\+CPSI:\s*([^,]+),([^,]+),(\d+),(\d+),(-?\d+),(-?\d+),(-?\d+)/);
    if (!match) return null;

    return {
      system: match[1].trim(),
      operation: match[2].trim(),
      bandwidth: parseInt(match[3], 10),
      rsrp: parseInt(match[4], 10),
      rsrq: parseInt(match[5], 10),
      snr: parseInt(match[6], 10),
    };
  }
}

// Singleton instance
export const atParser = new ATParser();
