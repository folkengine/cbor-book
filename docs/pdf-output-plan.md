# PDF Output — Design & Implementation Plan (deferred)

**Status:** Deferred — not yet implemented. Captured 2026-06-25.
**Depends on:** the EPUB output work (`[output.epub]` in `book.toml`) already
landed/in-flight.

This document records the agreed approach for adding a PDF rendering of the
book so it can be picked up later without re-deriving the analysis.

## Goal

Produce a downloadable PDF of *The CBOR, dCBOR, and Gordian Envelope Book*.

Phased, so we get a working artifact quickly and improve typography later:

- **Phase 1 (first):** a *basic but faithful* PDF — correct content and
  styling, but accept rough page breaks and no page numbers/running headers.
- **Phase 2 (later):** polish via print CSS — page-break control, page
  numbers, running headers, a print table of contents, optional cover page.

## Recommended approach: print `print.html` with a headless browser

mdBook has **no native PDF backend**. The chosen route is to print the HTML
backend's existing `print.html` (the whole book concatenated onto one page,
generated automatically by `[output.html]`) to PDF using headless Chrome /
Chromium.

Key path detail: because this book configures **multiple renderers**, the HTML
output lands under `book/html/`, so the print page is at:

```
book/html/print.html
```

### Why this route, and not Pandoc/LaTeX or Typst

This is the decisive point and the reason the "highest quality typesetting"
engine is the *wrong* tool here:

- The book's content is produced by **HTML-emitting preprocessors** —
  `mdbook-dcbor-shiki` injects styled `<pre>`/`<span>` markup, `mdbook-admonish`
  injects styled `<div>`s, all driven by `theme/*.css`.
- Any approach that **reuses the rendered HTML** (headless-browser print)
  inherits that styling for free and matches the published website.
- `mdbook-pandoc` does the opposite: it re-parses Markdown and converts to
  LaTeX/Typst. The HTML our preprocessors emit would arrive as raw HTML
  mid-Markdown and likely be mangled or dropped — *worse* fidelity, plus a
  heavy CI toolchain (Pandoc + a TeX distribution).

This is the same reasoning that makes `pagetoc` `renderers = ["html"]`: the
toolchain is HTML-centric, so stay in HTML.

### Alternatives considered

| Approach | Mechanism | CI deps | Verdict |
|---|---|---|---|
| **Headless Chrome on `print.html`** | Print existing HTML to PDF | Chromium | **Chosen** — highest fidelity, no `book.toml` change |
| `mdbook-pdf` backend | Drives headless Chromium for you | Chromium + plugin | Same output, less control; extra dependency |
| `mdbook-pandoc` | Markdown → Pandoc → LaTeX/Typst → PDF | Pandoc + TeX (100s of MB) | Rejected — mangles HTML-preprocessed content |

## Phase 1 — implementation sketch

No `book.toml` change required: PDF is a **post-build step** over the existing
`print.html`.

1. Ensure the HTML build ran (`mdbook build`), producing `book/html/print.html`.
2. Print it to PDF with headless Chrome. Two options:
   - **Zero-install (local, macOS):** Chrome is already installed.
     ```bash
     "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
       --headless=new --no-pdf-header-footer \
       --print-to-pdf="book/pdf/cbor-book.pdf" \
       "book/html/print.html"
     ```
   - **Portable (CI-friendly):** a small Node script using Puppeteer for
     control over `printBackground`, margins, and `waitUntil: 'networkidle0'`
     (so syntax highlighting / fonts settle before printing). Preferred for
     Phase 2 because it exposes the knobs we'll need.
3. Output to `book/pdf/cbor-book.pdf` (mirrors `book/epub/`; `book/` is
   already gitignored).
4. Add a `just pdf` recipe wrapping build + print.

### Gotchas to remember

- Use `--headless=new`; old headless had limited print support.
- Enable **print backgrounds** (Puppeteer `printBackground: true`) or admonition
  / code-block background colors disappear.
- Wait for the page to be idle before printing, or highlighting/fonts may be
  missing.
- `print.html` is under `book/html/`, not `book/`, due to multiple renderers.

## Phase 2 — polish (print CSS)

There is currently **no `@media print` CSS** in `theme/custom.css`. Phase 2
work, each item independent and low-risk:

- `break-inside: avoid` on code blocks (`pre`) and admonitions to stop
  mid-block splits.
- `@page` rules for margins and page numbers (`@bottom-center { content: counter(page) }`).
- Running headers (chapter title) via `@page` named pages, if supported by the
  chosen engine (Chrome's `@page` support is partial — may need Puppeteer
  `headerTemplate`/`footerTemplate`).
- A print-only cover page and TOC.
- Hide web-only chrome (sidebar toggles, nav buttons) in print.

## Invocation surfaces (open question)

Decide how far to wire it in. Not mutually exclusive; suggested order:

1. **Local `just pdf`** — run on demand. Simplest first step.
2. **CI artifact** — GitHub Action builds the PDF and uploads it (CI must
   install Chromium). Pairs naturally with the planned HTML+EPUB validation
   workflow.
3. **Published** — wire into `./deploy` so a PDF link goes live on
   cborbook.com alongside the book.

## Validation (when added to CI)

Beyond "file exists":

- `pdfinfo` or `qpdf --check` to confirm a structurally valid, non-truncated
  PDF and report page count (catch a 0-page / partial render).
- Upload as a build artifact for visual spot-checks.

## Open questions to resolve before implementing

1. Quality bar for the *first* shipped PDF — confirmed: basic/readable first,
   polish later.
2. Invocation surface — local only, CI artifact, and/or published? (see above)
3. Tooling — zero-install system Chrome vs a committed Puppeteer script
   (recommended for Phase 2 control). If Puppeteer, add it to `package.json`.
4. Filename/branding — `cbor-book.pdf` vs the full title; cover page?

## Out of scope

- Print-perfect typesetting in Phase 1.
- Replacing the HTML/EPUB pipeline — PDF is additive and independent.
