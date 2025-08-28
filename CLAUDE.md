# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is the Polkadot Blockchain Academy (PBA) content repository, containing educational materials for teaching blockchain technology, Substrate, and Polkadot. The repository uses Reveal.js via reveal-md for creating interactive slide presentations.

## Common Development Commands

### Running the Slide Server
```bash
yarn start        # Starts reveal-md server with hot reload on http://localhost:1948
```

### Building Static Files
```bash
yarn build        # Build static slides to ./build
yarn build:all    # Build slides and agenda app, copy dist/* to build/
yarn build:agenda # Build only the agenda React app
yarn clean        # Remove build directory
```

### Linting and Formatting
```bash
yarn lint         # Check markdown formatting with Prettier
yarn lint:write   # Auto-format all markdown files
yarn lint:agenda  # Check React/TSX formatting
yarn lint-fix:agenda # Auto-format React/TSX files
```

### Testing and Validation
```bash
yarn links <file> # Check markdown links in a specific file
yarn mod-links <module-number> # Check links for a specific module (e.g., yarn mod-links 1)
```

### PDF Generation
```bash
yarn generate:pdfs    # Generate PDFs from all slides using Playwright
yarn regenerate:pdfs  # Complete regeneration: clean, pull latest, generate, verify
yarn verify:pdfs      # Verify PDF content matches web-rendered content (with timeout wrapper)
yarn verify:pdfs:direct # Direct verification without timeout wrapper
yarn test:pdf         # Test PDF generation with specific problematic files for debugging
yarn clean:pdfs       # Remove all generated PDFs
```

### Cohort Tag Management
```bash
yarn tag:cambridge    # Build Cambridge cohort version
yarn tag:buenos-aires # Build Buenos Aires cohort version
yarn tag:berkeley     # Build Berkeley cohort version
yarn tag:hong-kong    # Build Hong Kong cohort version
yarn tag:singapore    # Build Singapore cohort version
yarn tag:lucerne      # Build Lucerne cohort version
yarn tag:current      # Build current main branch version
yarn prepare-deployment # Build all cohort versions for deployment
yarn deploy           # Deploy to GitHub Pages
```

### Additional Development Commands
```bash
yarn extract          # Extract syllabus structure to JSON for agenda app
yarn serve            # Serve built static files locally on port 1949
yarn remove-notes <module> # Remove speaker notes from slides (e.g., yarn remove-notes 2-Economics)
```

## Architecture

### Content Structure
- **syllabus/**: Contains all course modules organized by numbered directories
  - Slide files follow pattern `*-slides.md` (discovered via glob pattern in reveal-md.json)
  - Uses reveal-md format: `---` for horizontal slides, `---v` for vertical slides
  - Front matter YAML for metadata (title, etc.)
  - Speaker notes marked with `Notes:` keyword
- **assets/**: Presentation assets and configuration
  - `styles/PBA-theme.css`: Custom theme for reveal.js
  - `styles/custom-classes.css`: Additional custom CSS
  - `img/`: Organized image assets by module
  - `plugin/`: reveal.js plugins (mermaid, chart.js, tailwind)
  - `templates/`: HTML templates for slides and listing pages
- **src/**: React-based agenda application
  - Built with Vite and React 18
  - TypeScript for type safety
  - `extractSyllabus.ts`: Parses markdown files to generate syllabus.json
- **scripts/**: PDF generation utilities
  - `generate-pdfs-playwright.js`: Main PDF generation using Playwright
  - `regenerate-pdfs.js`: Full regeneration workflow
  - `verify-pdfs.js`: Validation of generated PDFs
- **pdfs/**: Generated PDF output directory (gitignored)
- **previous_syllabus/**: Archive of content from previous cohorts

### Key Configuration Files
- **reveal-md.json**: Core reveal-md configuration
  - Theme and CSS paths
  - Slide separator patterns
  - Glob pattern for slide discovery (`./syllabus/**/*[Ss]lides.md`)
  - Static build directory configuration
- **vite.config.ts**: Vite bundler configuration for agenda app
  - React SWC plugin for fast refresh
  - SVG support via vite-plugin-svgr
  - Build output to `dist/` directory
- **.nvmrc**: Node version specification (use with `nvm install`)
- **package.json**: Dependencies and all script definitions
  - Uses Yarn 3.2.1 as package manager
  - Key deps: reveal-md, react, vite, playwright

### Slide Authoring Guidelines
- Markdown files with reveal.js extensions
- YAML front matter for metadata
- Speaker notes with `Notes:` blocks (press 's' when presenting)
- Custom HTML elements for layout:
  - `<pba-cols>` and `<pba-col>` for column layouts
  - Can use `<!-- prettier-ignore -->` to prevent formatting issues
- Image paths relative to repository root (e.g., `/assets/img/...`)
- Highlight theme: shades-of-purple

### PDF Generation Process
- Uses Playwright for headless browser PDF generation
- Processes files in batches to manage memory
- Maintains syllabus directory structure in output
- Options: A4 paper size, custom theme application
- Test mode available for quick verification

### Deployment
- Multiple cohort versions via git tags
- GitHub Pages deployment workflow
- Static builds preserve all assets
- Front page in `frontpage/` directory
- Each cohort gets its own subdirectory in deployment

## Important Notes
- Always use `yarn` (not npm) for dependency management
- Enable corepack for Yarn 3: `corepack enable`
- Prettier formatting enforced via CI
- Link checking in CI using markdown-link-check
- Repository supports both local development and static deployment