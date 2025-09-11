#!/usr/bin/env node

/**
 * Generate a single PDF with speaker notes support
 */

const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');

const slideFile = process.argv[2];
if (!slideFile) {
  console.error('Usage: node generate-single-pdf.js <slide-file>');
  process.exit(1);
}

async function generatePDF() {
  console.log('Generating PDF for:', slideFile);
  
  const { spawn } = require('child_process');
  const server = spawn('yarn', ['start'], {
    stdio: 'pipe',
    shell: true
  });
  
  await new Promise(resolve => setTimeout(resolve, 5000));
  
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext();
  const page = await context.newPage();
  
  try {
    const url = `http://localhost:1948/${slideFile}?print-pdf&showNotes=separate-page`;
    console.log('Loading:', url);
    
    await page.goto(url, {
      waitUntil: 'networkidle',
      timeout: 30000
    });
    
    await page.waitForSelector('.reveal .slides', { timeout: 10000 });
    await page.waitForTimeout(3000);
    
    // Configure reveal.js for better PDF output with speaker notes
    await page.evaluate(() => {
      if (window.Reveal) {
        window.Reveal.configure({ 
          pdfMaxPagesPerSlide: 1,
          pdfSeparateFragments: false,
          showNotes: 'separate-page',
          controls: false,
          progress: false,
          center: true
        });
        
        // Remove any overlay elements
        const overlays = document.querySelectorAll('.reveal .overlay, .reveal .pause-overlay');
        overlays.forEach(el => el.style.display = 'none');
        
        const controls = document.querySelector('.reveal .controls');
        if (controls) controls.style.display = 'none';
        
        const progressBar = document.querySelector('.reveal .progress');
        if (progressBar) progressBar.style.display = 'none';
      }
    });
    
    await page.waitForTimeout(2000);
    
    // Add CSS to ensure clean PDF output
    await page.addStyleTag({
      content: `
        @media print {
          .reveal .controls,
          .reveal .progress,
          .reveal .playback,
          .reveal .pause-overlay,
          .reveal .overlay {
            display: none !important;
          }
          
          /* Ensure speaker notes are visible with proper styling */
          .reveal .speaker-notes {
            display: block !important;
            page-break-before: always;
            background: white !important;
            color: black !important;
          }
          
          /* Keep original slide backgrounds - DO NOT FORCE WHITE */
          /* The slides have dark backgrounds with light text by design */
          
          /* Remove shadows from code blocks but keep original colors */
          .reveal pre {
            box-shadow: none !important;
          }
        }
      `
    });
    
    const relativePath = path.relative('syllabus', slideFile);
    const outputPath = path.join('pdfs', relativePath.replace('.md', '.pdf'));
    
    // Create output directory
    const outputDir = path.dirname(outputPath);
    if (!fs.existsSync(outputDir)) {
      fs.mkdirSync(outputDir, { recursive: true });
    }
    
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
      preferCSSPageSize: false,
      displayHeaderFooter: false
    });
    
    const stats = fs.statSync(outputPath);
    console.log('✅ Generated:', outputPath, Math.round(stats.size / 1024) + 'KB');
    
  } catch (error) {
    console.error('Failed:', error.message);
  }
  
  await context.close();
  await browser.close();
  server.kill();
}

generatePDF();