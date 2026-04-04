/**
 * Output Formatter - JSON and Human-readable formatting
 */

import type { OutputFormat } from '../core/types/index.js';

export function formatJson(data: unknown): string {
  return JSON.stringify(data, null, 2);
}

export function formatHuman(data: Record<string, unknown>): string {
  const lines: string[] = [];

  for (const [key, value] of Object.entries(data)) {
    if (value === null || value === undefined) continue;

    if (typeof value === 'object' && !Array.isArray(value)) {
      lines.push(`${key}:`);
      for (const [subKey, subValue] of Object.entries(value as Record<string, unknown>)) {
        lines.push(`  ${subKey}: ${subValue}`);
      }
    } else if (Array.isArray(value)) {
      lines.push(`${key}:`);
      for (const item of value) {
        lines.push(`  - ${JSON.stringify(item)}`);
      }
    } else {
      lines.push(`${key}: ${value}`);
    }
  }

  return lines.join('\n');
}

export function formatOutput<T>(
  data: T,
  format: OutputFormat
): string {
  if (format === 'json') {
    return formatJson(data);
  }

  if (typeof data === 'object' && data !== null) {
    return formatHuman(data as Record<string, unknown>);
  }

  return String(data);
}
