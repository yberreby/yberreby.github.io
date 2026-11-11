# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a personal blog/website built with Zola, a fast static site generator written in Rust, and deployed to GitHub Pages.

The Zola version is pinned by the deploy action tag in `.github/workflows/main.yml` (`shalzz/zola-deploy-action@vX.Y.Z` is Zola X.Y.Z). Build locally with the same version (`zola --version`), and bump the tag when the local Zola changes. Zola 0.22 and 0.23 were breaking releases (highlighting config, Tera 2, shortcodes removed), so a mismatch can fail the build or change the output silently.

## Essential Commands

### Development
- `zola serve` - Start local development server with live reload (default: http://127.0.0.1:1111)
- `zola build` - Build static site to `public/` directory
- `zola check` - Verify all internal links are valid

### Git & Deployment
- Two remotes: `origin` is the private repository; `deploy` is the public `yberreby/yberreby.github.io`
- Publishing is `git push deploy master`: GitHub Actions in the public repository builds the site and pushes it to its `gh-pages` branch, served at https://yberreby.com. Pushing to `origin` does not deploy
- Keep `origin/master` and `deploy/master` at the same commit

## Architecture & Key Patterns

### Content Structure
- **Blog posts**: `content/posts/YYYY-MM-DD-slug/index.md` - Each post in its own directory with assets
- **Static pages**: `content/pages/*.md` - Single file pages (resources, hackathons, demos)
- **Homepage**: `content/_index.md` - Main landing page content

### Templating System
- Tera 2 templates, all in `templates/`; there is no theme
- Key templates:
  - `templates/index.html` - Base layout and homepage; every other template extends it
  - `templates/page.html` - Posts and pages, with the reference list
  - `templates/section.html` - Section listings
  - `templates/about.html` - Plain content pages (`template = "about.html"`)
  - `templates/components/` - Tera components (`post_title`, `katex`, `reference`, `bib_*`)
- Tera 2 has no macros or shortcodes. Components are global and called as `{{ <name arg={value} /> }}`, or `{% <name> %}body{% </name> %}` with a body
- Markdown content is itself a Tera template: components work in it, and literal `{{`, `{%` or `{#` must be wrapped in `{% raw %}...{% endraw %}`

### Styling
- SCSS compilation enabled via Zola
- Base styles in `sass/site.scss` and its `_*.scss` partials, from the Even theme (MIT, `licenses/even-theme-LICENSE`)
- Site-specific overrides in `sass/custom.scss`

### Key Configuration
- `config.toml` - Site configuration, syntax highlighting, taxonomies, menu
- KaTeX enabled for mathematical notation
- Taxonomies: categories and tags (with RSS feeds)

## Development Workflow

1. Content is written in Markdown with TOML frontmatter
2. Use `zola serve` for local development with hot reload
3. Commit changes to master branch
4. Push master to `origin` and to `deploy`; the push to `deploy` builds and publishes the site

## Special Features

- **Math Support**: KaTeX integration for LaTeX math rendering
- **Bibliography**: a `ref.bib` next to a post's `index.md` renders as a reference list; cite inline with `{{ <reference key="..." /> }}`
- **D3.js**: Data visualization support (see `content/pages/d3-demo.md`)
- **Responsive Design**: Mobile-friendly with Slideout.js menu

## Important Notes

- The layout, styles and `static/even.js` were absorbed from the Even theme (getzola/even at 994bfdb) when the site moved to Zola 0.23; they are ordinary site files now
- Static assets go in `static/` directory
- The site uses CDNs for external libraries (KaTeX, Slideout.js, D3.js)
- No formal testing framework - rely on `zola check` and visual inspection