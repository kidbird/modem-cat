#!/usr/bin/env bun
/**
 * Test script for modem connection and AT command
 */

import { SerialPort } from 'serialport';

async function main() {
  const portPath = 'COM18';
  const baudRate = 115200;

  console.log(`Connecting to ${portPath} at ${baudRate}...`);

  const port = new SerialPort({
    path: portPath,
    baudRate,
    dataBits: 8,
    parity: 'none',
    stopBits: 1,
    autoOpen: false,
  });

  return new Promise((resolve, reject) => {
    port.open((err) => {
      if (err) {
        console.error('Failed to open port:', err.message);
        reject(err);
        return;
      }

      console.log('Port opened, waiting for modem init...');

      let response = '';

      const onData = (chunk: Buffer) => {
        response += chunk.toString();
        console.log('RX:', JSON.stringify(chunk.toString()));
      };

      port.on('data', onData);

      port.on('error', (err) => {
        console.error('Port error:', err.message);
      });

      // Wait for modem to initialize
      setTimeout(() => {
        port.flush(() => {
          console.log('Sending AT...');
          const cmd = Buffer.from('AT\r\n');
          port.write(cmd, (err) => {
            if (err) {
              console.error('Write error:', err.message);
              port.close();
              reject(err);
              return;
            }

            // Wait for response
            setTimeout(() => {
              port.removeListener('data', onData);
              console.log('\nResponse:', JSON.stringify(response));

              if (response.includes('OK')) {
                console.log('\n✅ SUCCESS: AT command returned OK');
                port.close(() => resolve(true));
              } else {
                console.log('\n❌ FAILED: No OK in response');
                port.close(() => resolve(false));
              }
            }, 3000);
          });
        });
      }, 3000);
    });
  });
}

main().then((success) => {
  console.log(success ? '\nTest PASSED' : '\nTest FAILED');
  process.exit(success ? 0 : 1);
}).catch((err) => {
  console.error('Error:', err);
  process.exit(1);
});