/**
 * Modem Status Service
 * Query and manage modem status
 */

import type { ModemStatus, SignalStrength, NetworkRegistrationState, ConnectionMode } from '../types/index.js';

export class StatusService {
  /**
   * Get modem status
   */
  async getStatus(): Promise<ModemStatus> {
    // Mock implementation - would query actual modem
    return {
      registration: 'REGISTERED',
      signalStrength: {
        rsrp: -85,
        rsrq: -12,
      },
      connectionMode: '5G',
      imei: '123456789012345',
      operator: 'China Mobile',
    };
  }

  /**
   * Parse registration state from AT response
   */
  parseRegistration(state: number): NetworkRegistrationState {
    switch (state) {
      case 0: return 'NOT_REGISTERED';
      case 1: return 'REGISTERED';
      case 2: return 'SEARCHING';
      case 3: return 'DENIED';
      default: return 'NOT_REGISTERED';
    }
  }

  /**
   * Parse connection mode from AT response
   */
  parseConnectionMode(response: string): ConnectionMode {
    const lower = response.toLowerCase();
    if (lower.includes('5g') || lower.includes('nr')) {
      if (lower.includes('nsa')) return 'NSA';
      if (lower.includes('sa')) return 'SA';
      return '5G';
    }
    return '4G';
  }
}

export const statusService = new StatusService();
