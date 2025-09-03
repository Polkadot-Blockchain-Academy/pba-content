#!/usr/bin/env node

/**
 * Test script to verify a small subset of PDFs
 */

const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');
const pdfParse = require('pdf-parse');

// Test with just a few files
const TEST_FILES = [
  '1-Fundamentals/1-scale-slides.md',
  '2-Polkadot/a-1-polkadot-introduction-slides.md',
  '6-JAM/16-jam-services-slides.md'
];

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

async function extractPDFText(pdfPath) {
  try {
    const dataBuffer = fs.readFileSync(pdfPath);
    const data = await pdfParse(dataBuffer);
    return {
      text: data.text,
      pages: data.numpages
    };
  } catch (error) {
    logError(`Failed to parse PDF ${pdfPath}: ${error.message}`);
    return null;
  }
}

async function extractWebText(browser, slideFile, port) {
  let page = null;
  try {
    page = await browser.newPage();
    const url = `http://localhost:${port}/${slideFile}`;
    
    logInfo(`Loading ${url}`);
    
    await page.goto(url, {
      waitUntil: 'networkidle',
      timeout: 30000
    });
    
    // Wait for reveal.js to load
    await page.waitForSelector('.reveal .slides', { timeout: 10000 });
    await page.waitForTimeout(2000);
    
    // Get slide count
    const slideCount = await page.evaluate(() => {
      return document.querySelectorAll('.reveal .slides section').length;
    });
    
    // Get text content
    const text = await page.evaluate(() => {
      const slides = document.querySelectorAll('.reveal .slides section');
      const texts = [];
      slides.forEach(slide => {
        const slideClone = slide.cloneNode(true);
        const notes = slideClone.querySelector('aside.notes');
        if (notes) notes.remove();
        const text = slideClone.innerText || slideClone.textContent || '';
        if (text.trim()) {
          texts.push(text.trim());
        }
      });
      return texts.join('\n\n');
    });
    
    await page.close();
    
    return {
      text: text,
      slideCount: slideCount
    };
  } catch (error) {
    logError(`Failed to extract web content: ${error.message}`);
    if (page) {
      try {
        await page.close();
      } catch (e) {}
    }
    return null;
  }
}

async function testVerification() {
  log('🔍 Testing PDF Verification', 'bright');
  log('');
  
  // Start server
  const { spawn } = require('child_process');
  
  logInfo('Starting reveal-md server...');
  const server = spawn('yarn', ['start', '--port', '9001'], {
    stdio: 'pipe',
    shell: true
  });
  
  // Wait for server to start
  await new Promise(resolve => {
    const timeout = setTimeout(resolve, 5000);
    server.stdout.on('data', (data) => {
      if (data.toString().includes('Reveal-server started')) {
        clearTimeout(timeout);
        setTimeout(resolve, 2000);
      }
    });
  });
  
  logSuccess('Server started on port 9001');
  
  const browser = await chromium.launch({ headless: true });
  logSuccess('Browser launched');
  
  log('');
  log('Testing verification with sample files...', 'cyan');
  log('');
  
  for (const file of TEST_FILES) {
    const slideFile = `syllabus/${file}`;
    const pdfFile = `pdfs/${file.replace('.md', '.pdf')}`;
    
    log(`Testing: ${file}`, 'cyan');
    
    if (!fs.existsSync(pdfFile)) {
      logWarning(`PDF not found: ${pdfFile}`);
      continue;
    }
    
    const pdfData = await extractPDFText(pdfFile);
    const webData = await extractWebText(browser, slideFile, 9001);
    
    if (pdfData && webData) {
      const ratio = pdfData.text.length / webData.text.length;
      logInfo(`PDF pages: ${pdfData.pages}, Web slides: ${webData.slideCount}`);
      logInfo(`Content ratio: ${Math.round(ratio * 100)}%`);
      
      if (ratio < 0.5) {
        logWarning(`PDF has only ${Math.round(ratio * 100)}% of web content`);
      } else {
        logSuccess('Content looks good');
      }
    }
    
    log('');
  }
  
  // Cleanup
  await browser.close();
  server.kill();
  
  logSuccess('Test completed');
  process.exit(0);
}

if (require.main === module) {
  testVerification().catch(error => {
    logError(`Fatal error: ${error.message}`);
    console.error(error);
    process.exit(1);
  });
}