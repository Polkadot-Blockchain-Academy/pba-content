#!/usr/bin/env node

/**
 * PDF Verification Script
 * 
 * This script verifies that generated PDFs contain the expected content
 * by comparing them with the web-rendered versions of the presentations.
 */

const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');
const glob = require('glob');
const pdfParse = require('pdf-parse');

// Configuration
const SLIDES_PATTERN = 'syllabus/**/*slides.md';
const PDF_DIR = 'pdfs';
const PORT = 9000;

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

// Extract text from PDF
async function extractPDFText(pdfPath) {
  try {
    const dataBuffer = fs.readFileSync(pdfPath);
    const data = await pdfParse(dataBuffer);
    return {
      text: data.text,
      pages: data.numpages,
      info: data.info
    };
  } catch (error) {
    logError(`Failed to parse PDF ${pdfPath}: ${error.message}`);
    return null;
  }
}

// Extract text from web presentation
async function extractWebText(browser, slideFile, port) {
  try {
    const page = await browser.newPage();
    const url = `http://localhost:${port}/${slideFile}`;
    
    await page.goto(url, {
      waitUntil: 'networkidle',
      timeout: 30000
    });
    
    // Wait for reveal.js to load
    await page.waitForSelector('.reveal .slides', { timeout: 10000 });
    await page.waitForTimeout(2000);
    
    // Get all slide content
    const slideTexts = await page.evaluate(() => {
      const slides = document.querySelectorAll('.reveal .slides section');
      const texts = [];
      slides.forEach(slide => {
        // Clone the slide to not modify the original
        const slideClone = slide.cloneNode(true);
        
        // Remove speaker notes if present
        const notes = slideClone.querySelector('aside.notes');
        if (notes) notes.remove();
        
        // Remove any elements with class 'notes'
        const noteElements = slideClone.querySelectorAll('.notes');
        noteElements.forEach(el => el.remove());
        
        // Get visible text content
        const text = slideClone.innerText || slideClone.textContent || '';
        if (text.trim() && text.trim() !== 'Notes:') {
          texts.push(text.trim());
        }
      });
      return texts;
    });
    
    // Get slide count
    const slideCount = await page.evaluate(() => {
      return document.querySelectorAll('.reveal .slides section').length;
    });
    
    await page.close();
    
    return {
      texts: slideTexts,
      slideCount: slideCount,
      combinedText: slideTexts.join('\n\n')
    };
  } catch (error) {
    logError(`Failed to extract web content: ${error.message}`);
    return null;
  }
}

// Compare PDF and web content
function compareContent(pdfData, webData, file) {
  const results = {
    file: file,
    success: false,
    issues: [],
    stats: {}
  };
  
  if (!pdfData || !webData) {
    results.issues.push('Failed to extract content');
    return results;
  }
  
  // Basic statistics
  results.stats = {
    pdfPages: pdfData.pages,
    webSlides: webData.slideCount,
    pdfTextLength: pdfData.text.length,
    webTextLength: webData.combinedText.length
  };
  
  // Check if PDF has reasonable content
  if (pdfData.text.length < 100) {
    results.issues.push('PDF appears to be empty or nearly empty');
  }
  
  // Check if PDF has significantly less content than web
  const ratio = pdfData.text.length / webData.combinedText.length;
  if (ratio < 0.5) {
    results.issues.push(`PDF has only ${Math.round(ratio * 100)}% of web content`);
  }
  
  // Check for key content from each slide
  let missingSlides = 0;
  webData.texts.forEach((slideText, index) => {
    // Skip empty slides or slides with only notes markers
    if (slideText.length < 20 || slideText.trim() === 'Notes:') return;
    
    // Get first few meaningful words from slide
    const keywords = slideText.split(/\s+/)
      .filter(word => word.length > 3)
      .slice(0, 5);
    
    // Check if these keywords appear in PDF
    const found = keywords.some(keyword => 
      pdfData.text.toLowerCase().includes(keyword.toLowerCase())
    );
    
    if (!found && slideText.length > 50) {
      missingSlides++;
    }
  });
  
  if (missingSlides > 0) {
    const percentMissing = Math.round((missingSlides / webData.slideCount) * 100);
    if (percentMissing > 10) {
      results.issues.push(`~${percentMissing}% of slides may be missing from PDF`);
    }
  }
  
  // Overall success determination
  results.success = results.issues.length === 0 || 
    (results.issues.length === 1 && ratio > 0.8);
  
  return results;
}

// Main verification function
async function verifyPDFs() {
  log('🔍 PDF Content Verification', 'bright');
  log('Comparing generated PDFs with web-rendered content', 'cyan');
  log('');
  
  // Find all slide files
  const slideFiles = glob.sync(SLIDES_PATTERN, { absolute: false });
  logInfo(`Found ${slideFiles.length} slide files`);
  
  // Find corresponding PDFs
  const pdfFiles = slideFiles.map(slide => {
    const relativePath = path.relative('syllabus', slide);
    return path.join(PDF_DIR, relativePath.replace('.md', '.pdf'));
  }).filter(pdf => fs.existsSync(pdf));
  
  logInfo(`Found ${pdfFiles.length} generated PDFs`);
  
  if (pdfFiles.length === 0) {
    logError('No PDFs found to verify');
    return;
  }
  
  // Start server and browser
  const { spawn } = require('child_process');
  
  log('');
  logInfo('Starting reveal-md server...');
  const server = spawn('yarn', ['start', '--port', PORT.toString()], {
    stdio: 'pipe',
    shell: true
  });
  
  // Wait for server to start
  await new Promise(resolve => {
    server.stdout.on('data', (data) => {
      if (data.toString().includes('Reveal-server started')) {
        setTimeout(resolve, 2000);
      }
    });
  });
  logSuccess('Server started');
  
  const browser = await chromium.launch({ headless: true });
  logSuccess('Browser launched');
  
  log('');
  log('Verifying PDFs...', 'cyan');
  log('');
  
  const results = [];
  const sampleSize = pdfFiles.length; // Check ALL PDFs
  
  for (let i = 0; i < sampleSize; i++) {
    const pdfPath = pdfFiles[i];
    const slideFile = slideFiles.find(slide => {
      const relativePath = path.relative('syllabus', slide);
      const expectedPdf = path.join(PDF_DIR, relativePath.replace('.md', '.pdf'));
      return expectedPdf === pdfPath;
    });
    
    if (!slideFile) continue;
    
    const relativePath = path.relative('syllabus', slideFile);
    log(`Verifying: ${relativePath}`, 'cyan');
    
    // Extract content
    const pdfData = await extractPDFText(pdfPath);
    const webData = await extractWebText(browser, slideFile, PORT);
    
    // Compare
    const result = compareContent(pdfData, webData, relativePath);
    results.push(result);
    
    // Report
    if (result.success) {
      logSuccess(`${relativePath} - Content verified`);
      if (result.stats) {
        log(`  📄 ${result.stats.pdfPages} pages, ${result.stats.webSlides} slides`, 'blue');
      }
    } else {
      logWarning(`${relativePath} - Issues found:`);
      result.issues.forEach(issue => {
        log(`    - ${issue}`, 'yellow');
      });
    }
  }
  
  // Cleanup
  await browser.close();
  server.kill();
  
  // Summary
  log('');
  log('=' .repeat(50), 'bright');
  log('📊 VERIFICATION SUMMARY', 'bright');
  log('=' .repeat(50), 'bright');
  log('');
  
  const successful = results.filter(r => r.success);
  const failed = results.filter(r => !r.success);
  
  logSuccess(`Verified successfully: ${successful.length}/${sampleSize}`);
  
  if (failed.length > 0) {
    logWarning(`Issues found: ${failed.length} PDFs`);
    log('');
    log('PDFs with issues:', 'yellow');
    failed.forEach(result => {
      log(`  📄 ${result.file}`, 'yellow');
      result.issues.forEach(issue => {
        log(`     - ${issue}`, 'yellow');
      });
    });
  }
  
  // Overall statistics
  if (results.length > 0 && results[0].stats) {
    const avgPdfPages = results.reduce((sum, r) => sum + (r.stats?.pdfPages || 0), 0) / results.length;
    const avgWebSlides = results.reduce((sum, r) => sum + (r.stats?.webSlides || 0), 0) / results.length;
    
    log('');
    log('Average statistics:', 'cyan');
    log(`  📄 PDF pages: ${Math.round(avgPdfPages)}`, 'cyan');
    log(`  🖼️  Web slides: ${Math.round(avgWebSlides)}`, 'cyan');
  }
  
  log('');
  if (failed.length === 0) {
    log('✨ All sampled PDFs contain expected content!', 'green');
  } else {
    log('⚠️  Some PDFs may need regeneration', 'yellow');
  }
  
  // Exit properly
  process.exit(failed.length > 0 ? 1 : 0);
}

// Run verification
if (require.main === module) {
  verifyPDFs().then(() => {
    // Process will exit from within verifyPDFs
  }).catch(error => {
    logError(`Fatal error: ${error.message}`);
    console.error(error);
    process.exit(1);
  });
}