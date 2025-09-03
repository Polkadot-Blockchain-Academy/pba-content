# PDF Generation Scripts

This directory contains scripts for generating PDFs from all reveal.js presentations in the syllabus.

## Available Commands

### Test PDF Generation (Recommended First)
```bash
yarn generate:pdfs:test
```
This command generates PDFs for just 2 test files to verify the setup works correctly before processing all 73+ presentations.

### Generate All PDFs
```bash
yarn generate:pdfs
```
This command generates PDFs for all reveal.js presentations found in the `syllabus/` directory.

## What These Scripts Do

1. **Find Slide Files**: Automatically discovers all files ending with `-slides.md` in the syllabus directory
2. **Generate PDFs**: Uses reveal-md with the `--print` option to create PDFs
3. **Maintain Structure**: Preserves the directory structure from syllabus to the output directory
4. **Apply Styling**: Uses your custom PBA theme and CSS files
5. **Handle Errors**: Provides detailed feedback on success/failure for each file
6. **Progress Tracking**: Shows real-time progress with colored output

## Output

- **Test Mode**: PDFs are saved to `pdfs-test/` directory
- **Full Mode**: PDFs are saved to `pdfs/` directory
- Directory structure mirrors the syllabus structure
- Each `.md` file becomes a corresponding `.pdf` file

## Requirements

- `reveal-md` must be available via yarn
- Node.js with support for ES6+ features
- Sufficient disk space for all PDFs

## Configuration

The scripts use the following reveal-md options:
- `--print`: Generates PDF output
- `--print-size A4`: Sets paper size to A4
- `--theme assets/styles/PBA-theme.css`: Applies your custom theme
- `--css assets/styles/custom-classes.css`: Applies your custom CSS

## Troubleshooting

### Common Issues

1. **reveal-md not found**: Ensure `yarn install` has been run
2. **Permission errors**: Check file permissions on the scripts directory
3. **Memory issues**: The scripts process files in batches to avoid overwhelming the system
4. **PDF generation fails**: Check that the markdown files are valid and contain proper reveal.js syntax

### Getting Help

- Run `yarn help-rmd` to see all available reveal-md options
- Check the console output for specific error messages
- Start with the test command to isolate issues

## Performance Notes

- **Test Mode**: Processes 2 files sequentially
- **Full Mode**: Processes files in batches of 3 with 1-second delays between batches
- Total processing time depends on the number of slides and complexity of each presentation
- Consider running during off-peak hours for large batches

## File Structure Example

```
syllabus/
├── 1-Fundamentals/
│   ├── 1-scale-slides.md
│   └── 2-Runtime-slides.md
└── 2-Polkadot/
    └── some-slides.md

pdfs/ (or pdfs-test/)
├── 1-Fundamentals/
│   ├── 1-scale-slides.pdf
│   └── 2-Runtime-slides.pdf
└── 2-Polkadot/
    └── some-slides.pdf
```
