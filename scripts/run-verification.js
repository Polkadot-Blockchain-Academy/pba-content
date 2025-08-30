#!/usr/bin/env node

/**
 * Wrapper script to run PDF verification and ensure proper exit
 */

const { spawn } = require('child_process');

function runVerification() {
  return new Promise((resolve, reject) => {
    const verifyProcess = spawn('node', ['scripts/verify-pdfs.js'], {
      stdio: 'inherit',
      shell: false
    });

    verifyProcess.on('close', (code) => {
      resolve(code);
    });

    verifyProcess.on('error', (error) => {
      reject(error);
    });

    // Force exit after a timeout to prevent hanging
    const timeout = setTimeout(() => {
      console.log('\n⚠️  Verification taking too long, forcing exit...');
      verifyProcess.kill('SIGTERM');
      setTimeout(() => {
        verifyProcess.kill('SIGKILL');
        resolve(1);
      }, 5000);
    }, 120000); // 2 minute timeout

    // Clear timeout if process exits normally
    verifyProcess.on('exit', () => {
      clearTimeout(timeout);
    });
  });
}

async function main() {
  try {
    const exitCode = await runVerification();
    process.exit(exitCode);
  } catch (error) {
    console.error('Failed to run verification:', error);
    process.exit(1);
  }
}

if (require.main === module) {
  main();
}