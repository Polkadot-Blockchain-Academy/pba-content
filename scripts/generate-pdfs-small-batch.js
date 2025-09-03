#!/usr/bin/env node

/**
 * Generate PDFs for a small batch to test
 */

const { chromium } = require('playwright');
const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');
const glob = require('glob');

// Configuration - smaller batch
const SLIDES_PATTERN = 'syllabus/1-Fundamentals/*.md';
const OUTPUT_DIR = 'pdfs';
const PORT = 9600;

// Colors
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  cyan: '\x1b[36m'
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function logSuccess(message) { log(`✅ ${message}`, 'green'); }
function logError(message) { log(`❌ ${message}`, 'red'); }
function logInfo(message) { log(`ℹ️  ${message}`, 'blue'); }
function logProgress(message) { log(`🔄 ${message}`, 'cyan'); }

// Start server
function startRevealServer(port) {
  return new Promise((resolve, reject) => {
    const args = [
      'reveal-md',
      './',
      '--port', port.toString(),
      '--disable-auto-open',
      '--theme', 'assets/styles/PBA-theme.css',
      '--css', 'assets/styles/custom-classes.css'
    ];

    const server = spawn('yarn', args, {
      stdio: 'pipe',
      shell: true,
      cwd: process.cwd()
    });

    let serverStarted = false;
    
    server.stdout.on('data', (data) => {
      const output = data.toString();
      if (output.includes('Reveal-server started') && !serverStarted) {
        serverStarted = true;
        setTimeout(() => resolve(server), 3000);
      }
    });

    server.stderr.on('data', (data) => {
      const error = data.toString();
      if (!error.includes('DeprecationWarning')) {
        console.error(`Server stderr: ${error}`);
      }
    });

    setTimeout(() => {
      if (!serverStarted) {
        server.kill();
        reject(new Error('Server failed to start in time'));
      }
    }, 20000);
  });
}

// Generate PDF
async function generatePDF(browser, slideFile, port) {
  const relativePath = path.relative('syllabus', slideFile);
  const outputPath = path.join(OUTPUT_DIR, relativePath.replace('.md', '.pdf'));
  
  const outputDir = path.dirname(outputPath);
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  logProgress(`Generating: ${relativePath}`);

  try {
    const context = await browser.newContext();
    const page = await context.newPage();
    
    const url = `http://localhost:${port}/${slideFile}?print-pdf`;
    
    await page.goto(url, {
      waitUntil: 'networkidle',
      timeout: 60000
    });
    
    await page.waitForSelector('.reveal .slides', { timeout: 15000 });
    
    await page.waitForFunction(() => {
      return window.Reveal && window.Reveal.isReady();
    }, { timeout: 10000 });
    
    await page.evaluate(() => {
      if (window.Reveal) {
        window.Reveal.configure({ 
          pdfMaxPagesPerSlide: 1,
          pdfSeparateFragments: true,
          showNotes: false,
          transition: 'none',
          backgroundTransition: 'none'
        });
        window.Reveal.layout();
        window.Reveal.sync();
      }
    });
    
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(5000);
    
    await page.pdf({
      path: outputPath,
      format: 'A4',
      printBackground: true,
      margin: {
        top: '5mm',
        bottom: '5mm',
        left: '5mm',
        right: '5mm'
      },
      preferCSSPageSize: true,
      scale: 0.8
    });
    
    await context.close();
    
    if (fs.existsSync(outputPath)) {
      const stats = fs.statSync(outputPath);
      logSuccess(`${relativePath} → ${Math.round(stats.size / 1024)}KB`);
      return { success: true, file: relativePath, size: stats.size };
    } else {
      throw new Error('PDF not created');
    }
    
  } catch (error) {
    logError(`Failed ${relativePath}: ${error.message}`);
    return { success: false, file: relativePath, error: error.message };
  }
}

// Main
async function main() {
  log('🚀 Small Batch PDF Generation Test', 'bright');
  log('Testing with 1-Fundamentals module only', 'cyan');
  log('');

  if (!fs.existsSync(OUTPUT_DIR)) {
    fs.mkdirSync(OUTPUT_DIR, { recursive: true });
  }

  const slideFiles = glob.sync(SLIDES_PATTERN, { absolute: false });
  logInfo(`Found ${slideFiles.length} slide files in 1-Fundamentals`);
  
  if (slideFiles.length === 0) {
    logWarning('No slide files found');
    process.exit(0);
  }

  let server = null;
  let browser = null;
  const results = [];
  
  try {
    logInfo('Starting reveal-md server...');
    server = await startRevealServer(PORT);
    logSuccess('Server started');
    
    logInfo('Launching browser...');
    browser = await chromium.launch({
      headless: true,
      args: ['--no-sandbox', '--disable-setuid-sandbox']
    });
    logSuccess('Browser launched');
    
    log('');
    
    for (const file of slideFiles) {
      const result = await generatePDF(browser, file, PORT);
      results.push(result);
    }
    
  } catch (error) {
    logError(`Fatal error: ${error.message}`);
    console.error(error);
  } finally {
    if (browser) {
      await browser.close();
      logInfo('Browser closed');
    }
    if (server) {
      server.kill();
      logInfo('Server stopped');
    }
  }

  // Summary
  log('');
  log('=' .repeat(50), 'bright');
  log('📊 SUMMARY', 'bright');
  log('=' .repeat(50), 'bright');
  
  const successful = results.filter(r => r.success);
  const failed = results.filter(r => !r.success);
  
  logSuccess(`Generated: ${successful.length}/${slideFiles.length} PDFs`);
  
  if (failed.length > 0) {
    logError(`Failed: ${failed.length} files`);
    failed.forEach(r => log(`  - ${r.file}`, 'red'));
  }
  
  log('');
  log('✨ Test complete', 'green');
  process.exit(failed.length > 0 ? 1 : 0);
}

// Run
if (require.main === module) {
  main().catch(error => {
    logError(`Unhandled error: ${error.message}`);
    console.error(error);
    process.exit(1);
  });
}