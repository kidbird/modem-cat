/**
 * Config Service
 * Manage modem configuration
 */

export class ConfigService {
  private config: Record<string, string> = {
    networkMode: '5G',
    apn: 'cmnet',
    pin: '****',
  };

  /**
   * Get configuration
   */
  async get(key?: string): Promise<Record<string, string> | string> {
    if (key) {
      return this.config[key] || '';
    }
    return { ...this.config };
  }

  /**
   * Set configuration
   */
  async set(key: string, value: string): Promise<void> {
    this.config[key] = value;
  }

  /**
   * Reset to defaults
   */
  async reset(): Promise<void> {
    this.config = {
      networkMode: '5G',
      apn: 'cmnet',
      pin: '****',
    };
  }
}

export const configService = new ConfigService();
