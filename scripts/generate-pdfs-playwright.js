#!/usr/bin/env node

/**
 * PDF Generation Script using Playwright
 * 
 * This script uses Playwright to generate PDFs from reveal.js presentations.
 * It ensures all slides are properly captured by using the print-pdf mode.
 */

const { chromium } = require('playwright');
const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');
const glob = require('glob');

// Configuration
const SLIDES_PATTERN = 'syllabus/**/*slides.md';
const OUTPUT_DIR = 'pdfs';
const BASE_PORT = 9500;
const BATCH_SIZE = 5; // Process in batches to avoid resource exhaustion

// Colors for console output
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
function logWarning(message) { log(`⚠️  ${message}`, 'yellow'); }
function logProgress(message) { log(`🔄 ${message}`, 'cyan'); }

// Ensure output directory exists
function ensureOutputDir() {
  if (!fs.existsSync(OUTPUT_DIR)) {
    fs.mkdirSync(OUTPUT_DIR, { recursive: true });
    logSuccess(`Created output directory: ${OUTPUT_DIR}`);
  }
}

// Find all slide files
function findSlideFiles() {
  try {
    const files = glob.sync(SLIDES_PATTERN, { absolute: false });
    logInfo(`Found ${files.length} slide files`);
    return files;
  } catch (error) {
    logError(`Error finding slide files: ${error.message}`);
    process.exit(1);
  }
}

// Start reveal-md server for batch processing
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
        setTimeout(() => resolve(server), 3000); // Give server time to fully initialize
      }
    });

    server.stderr.on('data', (data) => {
      const error = data.toString();
      if (!error.includes('DeprecationWarning')) {
        console.error(`Server stderr: ${error}`);
      }
    });

    server.on('error', (error) => {
      reject(error);
    });

    // Timeout if server doesn't start
    setTimeout(() => {
      if (!serverStarted) {
        server.kill();
        reject(new Error('Server failed to start in time'));
      }
    }, 30000);
  });
}

// Generate PDF using Playwright
async function generatePDFWithPlaywright(browser, slideFile, port) {
  const relativePath = path.relative('syllabus', slideFile);
  const outputPath = path.join(OUTPUT_DIR, relativePath.replace('.md', '.pdf'));
  
  // Create output directory
  const outputDir = path.dirname(outputPath);
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  logProgress(`Generating: ${relativePath}`);

  try {
    const context = await browser.newContext();
    const page = await context.newPage();
    
    // Navigate to the presentation with print-pdf parameter
    const url = `http://localhost:${port}/${slideFile}?print-pdf`;
    
    await page.goto(url, {
      waitUntil: 'networkidle',
      timeout: 30000
    });
    
    // Wait for reveal.js to initialize and slides to render in print mode
    await page.waitForSelector('.reveal .slides', { timeout: 10000 });
    
    // Wait a bit more for all content to render
    await page.waitForTimeout(3000);
    
    // Check if slides are in print mode
    const isPrintMode = await page.evaluate(() => {
      const reveal = document.querySelector('.reveal');
      return reveal && reveal.classList.contains('print-pdf');
    });
    
    if (!isPrintMode) {
      // Try to trigger print mode manually
      await page.evaluate(() => {
        if (window.Reveal) {
          window.Reveal.configure({ 
            pdfMaxPagesPerSlide: 1,
            pdfSeparateFragments: false
          });
        }
      });
      await page.waitForTimeout(2000);
    }
    
    // Generate PDF with proper settings
    await page.pdf({
      path: outputPath,
      format: 'A4',
      printBackground: true,
      margin: {
        top: '10mm',
        bottom: '10mm',
        left: '10mm',
        right: '10mm'
      },
      preferCSSPageSize: false // Use our format instead of CSS page size
    });
    
    await context.close();
    
    // Verify PDF was created and has reasonable size
    if (fs.existsSync(outputPath)) {
      const stats = fs.statSync(outputPath);
      if (stats.size > 50000) { // Expect at least 50KB
        logSuccess(`${relativePath} → ${Math.round(stats.size / 1024)}KB`);
        return { success: true, file: relativePath, size: stats.size };
      } else {
        logWarning(`Small PDF: ${relativePath} (${Math.round(stats.size / 1024)}KB)`);
        return { success: true, file: relativePath, size: stats.size, warning: 'Small size' };
      }
    } else {
      throw new Error('PDF not created');
    }
    
  } catch (error) {
    logError(`Failed ${relativePath}: ${error.message}`);
    return { success: false, file: relativePath, error: error.message };
  }
}

// Process a batch of files
async function processBatch(browser, files, server, port) {
  const results = [];
  
  for (const file of files) {
    const result = await generatePDFWithPlaywright(browser, file, port);
    results.push(result);
  }
  
  return results;
}

// Main function
async function main() {
  log('🚀 Starting PDF Generation with Playwright', 'bright');
  log('This will capture all slides in each presentation', 'cyan');
  log('');

  ensureOutputDir();
  const slideFiles = findSlideFiles();
  
  if (slideFiles.length === 0) {
    logWarning('No slide files found');
    process.exit(0);
  }

  log('');
  logInfo(`Processing ${slideFiles.length} presentations in batches of ${BATCH_SIZE}...`);
  log('');

  const results = [];
  let port = BASE_PORT;
  let server = null;
  let browser = null;
  
  try {
    // Start server
    logInfo('Starting reveal-md server...');
    server = await startRevealServer(port);
    logSuccess('Server started successfully');
    
    // Launch browser
    logInfo('Launching browser...');
    browser = await chromium.launch({
      headless: true,
      args: ['--no-sandbox', '--disable-setuid-sandbox']
    });
    logSuccess('Browser launched');
    
    log('');
    
    // Process files in batches
    for (let i = 0; i < slideFiles.length; i += BATCH_SIZE) {
      const batch = slideFiles.slice(i, Math.min(i + BATCH_SIZE, slideFiles.length));
      const batchNum = Math.floor(i / BATCH_SIZE) + 1;
      const totalBatches = Math.ceil(slideFiles.length / BATCH_SIZE);
      
      logInfo(`Processing batch ${batchNum}/${totalBatches}...`);
      
      const batchResults = await processBatch(browser, batch, server, port);
      results.push(...batchResults);
      
      // Progress
      const progress = Math.round(Math.min(i + BATCH_SIZE, slideFiles.length) / slideFiles.length * 100);
      log(`Overall progress: ${progress}%`, 'cyan');
      log('');
      
      // Small delay between batches
      if (i + BATCH_SIZE < slideFiles.length) {
        await new Promise(resolve => setTimeout(resolve, 1000));
      }
    }
    
  } catch (error) {
    logError(`Fatal error: ${error.message}`);
    console.error(error);
  } finally {
    // Cleanup
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
  log('📊 GENERATION SUMMARY', 'bright');
  log('=' .repeat(50), 'bright');
  log('');

  const successful = results.filter(r => r.success);
  const warnings = results.filter(r => r.warning);
  const failed = results.filter(r => !r.success);

  logSuccess(`Successfully generated: ${successful.length}/${slideFiles.length} PDFs`);
  
  if (warnings.length > 0) {
    logWarning(`Files with warnings: ${warnings.length}`);
  }
  
  if (successful.length > 0) {
    const totalSize = successful.reduce((sum, r) => sum + (r.size || 0), 0);
    logInfo(`Total size: ${Math.round(totalSize / 1024 / 1024)}MB`);
    logInfo(`Average size: ${Math.round(totalSize / successful.length / 1024)}KB per PDF`);
    
    // Group by module
    log('');
    log('Modules generated:', 'cyan');
    const modules = {};
    successful.forEach(r => {
      const module = r.file.split('/')[0];
      if (!modules[module]) modules[module] = { count: 0, size: 0 };
      modules[module].count++;
      modules[module].size += r.size || 0;
    });
    
    Object.keys(modules).sort().forEach(module => {
      const avgSize = Math.round(modules[module].size / modules[module].count / 1024);
      log(`  📁 ${module}: ${modules[module].count} PDFs (avg ${avgSize}KB)`, 'cyan');
    });
  }
  
  if (failed.length > 0) {
    log('');
    logError(`Failed: ${failed.length} files`);
    log('Failed files:', 'red');
    failed.forEach(result => {
      log(`  ❌ ${result.file}`, 'red');
      if (result.error) {
        log(`     ${result.error.substring(0, 100)}`, 'yellow');
      }
    });
  }

  log('');
  log('=' .repeat(50), 'bright');
  log(`📁 PDFs saved to: ${OUTPUT_DIR}/`, 'bright');
  
  if (failed.length === 0) {
    log('');
    log('🎉 ALL PDFs GENERATED SUCCESSFULLY!', 'green');
  } else if (successful.length > 0) {
    log('');
    log('⚠️  Partial success. Check failed files above.', 'yellow');
  } else {
    log('');
    logError('Generation failed completely.');
    process.exit(1);
  }
  
  process.exit(failed.length > 0 ? 1 : 0);
}

// Handle termination
process.on('SIGINT', () => {
  log('');
  logWarning('Interrupted by user');
  process.exit(130);
});

// Run
if (require.main === module) {
  main().catch(error => {
    logError(`Unhandled error: ${error.message}`);
    console.error(error);
    process.exit(1);
  });
}