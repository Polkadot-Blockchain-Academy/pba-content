#!/usr/bin/env node

/**
 * Test script to debug PDF generation issues
 * Tests a single problematic file to understand why content is missing
 */

const { chromium } = require('playwright');
const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

// Test with one of the problematic files
const TEST_FILES = [
  'syllabus/2-Polkadot/a-1-polkadot-introduction-slides.md',
  'syllabus/3a-Protocol_On-Chain/XCM/2.5-Cross-Consensus-Transfers-slides.md'
];

const PORT = 8888;

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

// Start reveal-md server
function startServer(port) {
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
      console.log('Server output:', output);
      if (output.includes('Reveal-server started') && !serverStarted) {
        serverStarted = true;
        setTimeout(() => resolve(server), 5000); // Give more time
      }
    });

    server.stderr.on('data', (data) => {
      const error = data.toString();
      if (!error.includes('DeprecationWarning')) {
        console.error('Server error:', error);
      }
    });

    setTimeout(() => {
      if (!serverStarted) {
        reject(new Error('Server failed to start'));
      }
    }, 30000);
  });
}

async function testPDFGeneration(browser, slideFile, port) {
  const outputPath = `test-${path.basename(slideFile, '.md')}.pdf`;
  
  log(`\nTesting: ${slideFile}`, 'bright');
  log('=' .repeat(60), 'cyan');
  
  try {
    const context = await browser.newContext();
    const page = await context.newPage();
    
    // Test 1: Normal view
    logInfo('Testing normal view...');
    const normalUrl = `http://localhost:${port}/${slideFile}`;
    await page.goto(normalUrl, { waitUntil: 'networkidle', timeout: 30000 });
    
    await page.waitForSelector('.reveal .slides', { timeout: 10000 });
    
    const normalSlideCount = await page.evaluate(() => {
      const allSlides = document.querySelectorAll('.reveal .slides section');
      const visibleSlides = Array.from(allSlides).filter(slide => {
        const style = window.getComputedStyle(slide);
        return style.display !== 'none';
      });
      return {
        total: allSlides.length,
        visible: visibleSlides.length,
        hasReveal: !!window.Reveal,
        revealReady: window.Reveal ? window.Reveal.isReady() : false
      };
    });
    
    logInfo(`Normal view: ${normalSlideCount.visible}/${normalSlideCount.total} slides visible`);
    logInfo(`Reveal.js: ${normalSlideCount.hasReveal ? 'Loaded' : 'Not loaded'}, Ready: ${normalSlideCount.revealReady}`);
    
    // Test 2: Print-PDF view
    logInfo('\nTesting print-pdf view...');
    const printUrl = `http://localhost:${port}/${slideFile}?print-pdf`;
    await page.goto(printUrl, { waitUntil: 'networkidle', timeout: 30000 });
    
    // Wait for reveal to be ready
    await page.waitForFunction(() => {
      return window.Reveal && window.Reveal.isReady();
    }, { timeout: 15000 });
    
    // Force configuration
    await page.evaluate(() => {
      if (window.Reveal) {
        window.Reveal.configure({
          pdfMaxPagesPerSlide: 1,
          pdfSeparateFragments: true,
          showNotes: false
        });
        window.Reveal.layout();
        window.Reveal.sync();
      }
    });
    
    // Wait for layout to complete
    await page.waitForTimeout(5000);
    
    const printSlideCount = await page.evaluate(() => {
      const allSlides = document.querySelectorAll('.reveal .slides section');
      const visibleSlides = Array.from(allSlides).filter(slide => {
        const style = window.getComputedStyle(slide);
        const rect = slide.getBoundingClientRect();
        return style.display !== 'none' && 
               style.visibility !== 'hidden' &&
               rect.height > 0;
      });
      
      // Get first few slides content
      const slideContents = [];
      for (let i = 0; i < Math.min(5, allSlides.length); i++) {
        const slide = allSlides[i];
        const text = (slide.innerText || '').substring(0, 100);
        const style = window.getComputedStyle(slide);
        slideContents.push({
          index: i,
          text: text.replace(/\n/g, ' '),
          display: style.display,
          visibility: style.visibility,
          height: slide.getBoundingClientRect().height
        });
      }
      
      return {
        total: allSlides.length,
        visible: visibleSlides.length,
        isPrintMode: document.querySelector('.reveal').classList.contains('print-pdf'),
        slideContents
      };
    });
    
    logInfo(`Print-PDF view: ${printSlideCount.visible}/${printSlideCount.total} slides visible`);
    logInfo(`Print mode active: ${printSlideCount.isPrintMode}`);
    
    log('\nFirst 5 slides:', 'cyan');
    printSlideCount.slideContents.forEach(slide => {
      const status = slide.height > 0 ? '✓' : '✗';
      log(`  [${slide.index}] ${status} Height: ${slide.height}px, Display: ${slide.display}`, 'cyan');
      log(`      Content: "${slide.text.substring(0, 50)}..."`, 'cyan');
    });
    
    // Test 3: Generate PDF
    logInfo('\nGenerating PDF...');
    await page.pdf({
      path: outputPath,
      format: 'A4',
      printBackground: true,
      margin: { top: '5mm', bottom: '5mm', left: '5mm', right: '5mm' },
      preferCSSPageSize: true,
      scale: 0.8
    });
    
    // Check PDF size
    if (fs.existsSync(outputPath)) {
      const stats = fs.statSync(outputPath);
      const sizeKB = Math.round(stats.size / 1024);
      
      if (sizeKB > 100) {
        logSuccess(`PDF generated: ${outputPath} (${sizeKB}KB)`);
      } else {
        logWarning(`Small PDF generated: ${outputPath} (${sizeKB}KB)`);
      }
      
      // Try to parse PDF
      const pdfParse = require('pdf-parse');
      const dataBuffer = fs.readFileSync(outputPath);
      const pdfData = await pdfParse(dataBuffer);
      logInfo(`PDF has ${pdfData.numpages} pages`);
      logInfo(`Text length: ${pdfData.text.length} characters`);
    } else {
      logError('PDF not created');
    }
    
    await context.close();
    
  } catch (error) {
    logError(`Test failed: ${error.message}`);
    console.error(error);
  }
}

async function main() {
  log('🔬 PDF Generation Test', 'bright');
  log('Testing problematic files to debug missing content', 'cyan');
  log('');
  
  let server = null;
  let browser = null;
  
  try {
    // Start server
    logInfo('Starting reveal-md server...');
    server = await startServer(PORT);
    logSuccess('Server started');
    
    // Launch browser
    logInfo('Launching browser...');
    browser = await chromium.launch({
      headless: false, // Set to false to see what's happening
      devtools: true
    });
    logSuccess('Browser launched');
    
    // Test each file
    for (const file of TEST_FILES) {
      if (fs.existsSync(file)) {
        await testPDFGeneration(browser, file, PORT);
      } else {
        logWarning(`File not found: ${file}`);
      }
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
  
  log('');
  log('✨ Test complete', 'green');
  log('Check the generated test PDFs for content', 'cyan');
  process.exit(0);
}

// Run
if (require.main === module) {
  main().catch(error => {
    logError(`Unhandled error: ${error.message}`);
    console.error(error);
    process.exit(1);
  });
}