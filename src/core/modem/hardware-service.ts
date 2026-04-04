/**
 * Hardware Info Service
 * Query hardware information
 */

import type { HardwareInfo } from '../types/index.js';

export class HardwareService {
  /**
   * Get hardware information
   */
  async getHardwareInfo(): Promise<HardwareInfo> {
    // Mock implementation - would query actual modem
    return {
      model: 'MF269',
      firmwareVersion: 'V3.2.1',
      hardwareVersion: 'Rev.3',
      manufacturer: 'Fibocom',
      cpuUsage: 25,
      memoryUsage: 45,
      uptime: 86400,
    };
  }

  /**
   * Parse model from ATI response
   */
  parseModel(response: string): string {
    const lines = response.split('\n');
    for (const line of lines) {
      const trimmed = line.trim();
      if (trimmed && !trimmed.startsWith('AT')) {
        return trimmed;
      }
    }
    return 'Unknown';
  }
}

export const hardwareService = new HardwareService();
