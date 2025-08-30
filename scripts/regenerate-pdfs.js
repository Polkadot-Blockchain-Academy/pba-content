#!/usr/bin/env node

/**
 * Complete PDF Regeneration Script
 * 
 * This script performs a complete regeneration of all PDFs:
 * 1. Cleans existing PDFs
 * 2. Pulls latest content from GitHub
 * 3. Generates new PDFs
 * 4. Verifies the generated PDFs
 */

const { spawn, execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const glob = require('glob');
const pdfParse = require('pdf-parse');

// Configuration
const PDF_DIR = 'pdfs';
const SLIDES_PATTERN = 'syllabus/**/*slides.md';

// Colors for console output
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
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
function logStage(message) { 
  log(`\n${'='.repeat(60)}`, 'bright'); 
  log(`📋 ${message}`, 'bright'); 
  log(`${'='.repeat(60)}`, 'bright'); 
}

// Step 1: Clean existing PDFs
function cleanPDFs() {
  logStage('STEP 1: Cleaning existing PDFs');
  
  try {
    if (fs.existsSync(PDF_DIR)) {
      logProgress(`Removing existing ${PDF_DIR} directory...`);
      fs.rmSync(PDF_DIR, { recursive: true, force: true });
      logSuccess('Existing PDFs removed');
    } else {
      logInfo('No existing PDF directory found');
    }
    
    logProgress(`Creating fresh ${PDF_DIR} directory...`);
    fs.mkdirSync(PDF_DIR, { recursive: true });
    logSuccess('PDF directory created');
    
    return true;
  } catch (error) {
    logError(`Failed to clean PDFs: ${error.message}`);
    return false;
  }
}

// Step 2: Pull latest from GitHub
function pullLatestContent() {
  logStage('STEP 2: Pulling latest content from GitHub');
  
  try {
    // Check if we're in a git repository
    logProgress('Checking git status...');
    const status = execSync('git status --short', { encoding: 'utf8' });
    
    if (status.trim()) {
      logWarning('You have uncommitted changes:');
      console.log(status);
      logInfo('Stashing changes before pull...');
      execSync('git stash push -m "Auto-stash before PDF regeneration"');
      logSuccess('Changes stashed');
    }
    
    // Pull latest
    logProgress('Pulling latest from origin...');
    const pullResult = execSync('git pull origin $(git branch --show-current)', { encoding: 'utf8' });
    console.log(pullResult);
    logSuccess('Latest content pulled from GitHub');
    
    // Pop stash if we stashed
    if (status.trim()) {
      logInfo('Restoring stashed changes...');
      try {
        execSync('git stash pop');
        logSuccess('Stashed changes restored');
      } catch (e) {
        logWarning('Could not auto-restore stash, you may need to run "git stash pop" manually');
      }
    }
    
    return true;
  } catch (error) {
    logError(`Failed to pull latest content: ${error.message}`);
    logWarning('Continuing with current content...');
    return true; // Continue anyway
  }
}

// Step 3: Generate PDFs
function generatePDFs() {
  return new Promise((resolve) => {
    logStage('STEP 3: Generating PDFs');
    
    logProgress('Starting PDF generation process...');
    logInfo('This will take several minutes...');
    
    const generateProcess = spawn('node', ['scripts/generate-pdfs-playwright.js'], {
      stdio: 'pipe',
      shell: false
    });
    
    let pdfCount = 0;
    let lastProgress = '';
    
    generateProcess.stdout.on('data', (data) => {
      const output = data.toString();
      
      // Track successful PDFs
      const successMatches = output.match(/✅.*?→.*?KB/g);
      if (successMatches) {
        successMatches.forEach(match => {
          pdfCount++;
          console.log(`  [${pdfCount}] ${match}`);
        });
      }
      
      // Track progress percentage
      const progressMatch = output.match(/Overall progress: (\d+)%/);
      if (progressMatch && progressMatch[1] !== lastProgress) {
        lastProgress = progressMatch[1];
        logProgress(`Generation progress: ${lastProgress}%`);
      }
      
      // Show warnings
      if (output.includes('⚠️')) {
        console.log(output.trim());
      }
    });
    
    generateProcess.stderr.on('data', (data) => {
      const error = data.toString();
      if (!error.includes('DeprecationWarning') && !error.includes('ExperimentalWarning')) {
        logError(`Generation error: ${error}`);
      }
    });
    
    generateProcess.on('close', (code) => {
      if (code === 0) {
        logSuccess(`PDF generation complete! Generated ${pdfCount} PDFs`);
        resolve(true);
      } else {
        logError(`PDF generation failed with exit code ${code}`);
        resolve(false);
      }
    });
    
    generateProcess.on('error', (error) => {
      logError(`Failed to start generation process: ${error.message}`);
      resolve(false);
    });
  });
}

// Step 4: Verify PDFs
async function verifyPDFs() {
  logStage('STEP 4: Verifying generated PDFs');
  
  const KNOWN_PLACEHOLDERS = [
    'b-10-polkadot-cloud-and-hub-slides.md',
    'Presentation-slides.md',
    '11-inkubator-presentation-slides.md',
    '13-demystifying-jam-slides.md'
  ];
  
  try {
    // Find all expected PDFs
    const slideFiles = glob.sync(SLIDES_PATTERN);
    logInfo(`Found ${slideFiles.length} slide files to verify`);
    
    const results = {
      verified: 0,
      missing: [],
      tooSmall: [],
      errors: []
    };
    
    logProgress('Verifying each PDF...');
    
    for (const slideFile of slideFiles) {
      const relativePath = path.relative('syllabus', slideFile);
      const pdfPath = path.join(PDF_DIR, relativePath.replace('.md', '.pdf'));
      const isPlaceholder = KNOWN_PLACEHOLDERS.some(p => slideFile.includes(p));
      
      process.stdout.write(`  Checking ${relativePath.padEnd(60, '.')}`);
      
      if (!fs.existsSync(pdfPath)) {
        results.missing.push(relativePath);
        process.stdout.write(' ❌ Missing\n');
        continue;
      }
      
      try {
        const stats = fs.statSync(pdfPath);
        const sizeKB = Math.round(stats.size / 1024);
        
        // Quick content check
        const dataBuffer = fs.readFileSync(pdfPath);
        const pdfData = await pdfParse(dataBuffer);
        
        if (!isPlaceholder && sizeKB < 50) {
          results.tooSmall.push({ file: relativePath, size: sizeKB });
          process.stdout.write(` ⚠️ Small (${sizeKB}KB)\n`);
        } else if (pdfData.numpages < 1) {
          results.errors.push({ file: relativePath, error: 'No pages' });
          process.stdout.write(' ❌ No pages\n');
        } else {
          results.verified++;
          process.stdout.write(` ✅ OK (${sizeKB}KB, ${pdfData.numpages}p)\n`);
        }
      } catch (error) {
        results.errors.push({ file: relativePath, error: error.message });
        process.stdout.write(` ❌ Error\n`);
      }
    }
    
    // Summary
    log('');
    log('Verification Summary:', 'cyan');
    logSuccess(`Verified: ${results.verified}/${slideFiles.length} PDFs`);
    
    if (results.missing.length > 0) {
      logError(`Missing: ${results.missing.length} PDFs`);
      results.missing.forEach(f => log(`  - ${f}`, 'red'));
    }
    
    if (results.tooSmall.length > 0) {
      logWarning(`Suspiciously small: ${results.tooSmall.length} PDFs`);
      results.tooSmall.forEach(f => log(`  - ${f.file} (${f.size}KB)`, 'yellow'));
    }
    
    if (results.errors.length > 0) {
      logError(`Errors: ${results.errors.length} PDFs`);
      results.errors.forEach(f => log(`  - ${f.file}: ${f.error}`, 'red'));
    }
    
    return results.verified === slideFiles.length;
  } catch (error) {
    logError(`Verification failed: ${error.message}`);
    return false;
  }
}

// Main regeneration workflow
async function main() {
  const startTime = Date.now();
  
  log('');
  log('🚀 COMPLETE PDF REGENERATION WORKFLOW', 'bright');
  log('This will clean, update, generate, and verify all PDFs', 'cyan');
  
  let success = true;
  
  // Step 1: Clean
  if (!cleanPDFs()) {
    logError('Failed to clean PDFs');
    process.exit(1);
  }
  
  // Step 2: Pull latest
  if (!pullLatestContent()) {
    logWarning('Could not pull latest content, continuing anyway...');
  }
  
  // Step 3: Generate
  const generateSuccess = await generatePDFs();
  if (!generateSuccess) {
    logError('PDF generation failed');
    success = false;
  }
  
  // Step 4: Verify
  const verifySuccess = await verifyPDFs();
  if (!verifySuccess) {
    logWarning('Some PDFs failed verification');
    success = false;
  }
  
  // Final summary
  const elapsed = Math.round((Date.now() - startTime) / 1000);
  const minutes = Math.floor(elapsed / 60);
  const seconds = elapsed % 60;
  
  logStage('REGENERATION COMPLETE');
  
  if (success) {
    log('');
    logSuccess('✨ All PDFs regenerated successfully!');
    logInfo(`Time taken: ${minutes}m ${seconds}s`);
    
    // Final stats
    const pdfFiles = glob.sync(`${PDF_DIR}/**/*.pdf`);
    const totalSize = pdfFiles.reduce((sum, file) => {
      return sum + fs.statSync(file).size;
    }, 0);
    
    log('');
    log('📊 Final Statistics:', 'cyan');
    log(`  📁 Total PDFs: ${pdfFiles.length}`, 'cyan');
    log(`  💾 Total size: ${Math.round(totalSize / 1024 / 1024)}MB`, 'cyan');
    log(`  📍 Location: ${path.resolve(PDF_DIR)}/`, 'cyan');
  } else {
    log('');
    logWarning('⚠️  Regeneration completed with some issues');
    logInfo('Check the output above for details');
  }
  
  process.exit(success ? 0 : 1);
}

// Handle interruption
process.on('SIGINT', () => {
  log('');
  logWarning('Regeneration interrupted by user');
  process.exit(130);
});

// Run
if (require.main === module) {
  main().catch(error => {
    logError(`Fatal error: ${error.message}`);
    console.error(error);
    process.exit(1);
  });
}