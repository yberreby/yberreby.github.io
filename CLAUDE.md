# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a personal blog/website built with Zola (v0.21.0), a fast static site generator written in Rust. The site uses the "Even" theme and is automatically deployed to GitHub Pages.

## Essential Commands

### Development
- `zola serve` - Start local development server with live reload (default: http://127.0.0.1:1111)
- `zola build` - Build static site to `public/` directory
- `zola check` - Verify all internal links are valid

### Git & Deployment
- Deployment is automated via GitHub Actions on push to master branch
- The site deploys to the `gh-pages` branch and is served at https://yberreby.com

## Architecture & Key Patterns

### Content Structure
- **Blog posts**: `content/posts/YYYY-MM-DD-slug/index.md` - Each post in its own directory with assets
- **Static pages**: `content/pages/*.md` - Single file pages (resources, hackathons, demos)
- **Homepage**: `content/_index.md` - Main landing page content

### Templating System
- Uses Tera templates with inheritance from theme templates
- Custom templates override theme defaults in `templates/`
- Key templates:
  - `templates/index.html` - Homepage layout
  - `templates/page.html` - Generic page layout
  - `templates/section.html` - Section listings
  - `templates/bib_macros.html` - Bibliography/reference macros
  - `templates/shortcodes/reference.html` - Citation shortcode

### Styling
- SCSS compilation enabled via Zola
- Theme styles in `themes/even/sass/`
- Custom overrides in `sass/custom.scss`

### Key Configuration
- `config.toml` - Site configuration, theme settings, taxonomies
- KaTeX enabled for mathematical notation
- Taxonomies: categories and tags (with RSS feeds)

## Development Workflow

1. Content is written in Markdown with TOML frontmatter
2. Use `zola serve` for local development with hot reload
3. Commit changes to master branch
4. GitHub Actions automatically builds and deploys to GitHub Pages

## Special Features

- **Math Support**: KaTeX integration for LaTeX math rendering
- **Bibliography**: Custom reference system via shortcodes and macros
- **D3.js**: Data visualization support (see `content/pages/d3-demo.md`)
- **Responsive Design**: Mobile-friendly with Slideout.js menu

## Important Notes

- The theme is a git submodule - be careful when modifying theme files
- Static assets go in `static/` directory
- The site uses CDNs for external libraries (KaTeX, Slideout.js, D3.js)
- No formal testing framework - rely on `zola check` and visual inspection