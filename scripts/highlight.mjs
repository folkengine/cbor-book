#!/usr/bin/env node
import { createHighlighter } from 'shiki';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const projectRoot = path.dirname(__dirname);

const args = process.argv.slice(2);
let lang = 'envelope';
let isInlineMode = false;
let isBatchMode = false;

for (let i = 0; i < args.length; i++) {
  if (args[i] === '--inline') {
    isInlineMode = true;
  } else if (args[i] === '--batch-inline') {
    isBatchMode = true;
  } else {
    lang = args[i];
  }
}

// Function to strip JSON comments (JSONC -> JSON) and normalize whitespace
function stripJsonComments(jsonString) {
  // Remove single-line comments (// ...)
  let cleaned = jsonString.replace(/\/\/.*$/gm, '');
  // Convert tabs to spaces to avoid control character issues
  cleaned = cleaned.replace(/\t/g, '    ');
  // Remove any other problematic control characters except newlines and carriage returns
  cleaned = cleaned.replace(/[\x00-\x08\x0B\x0C\x0E-\x1F\x7F]/g, '');
  return cleaned;
}

// Function to resolve theme includes
function resolveThemeIncludes(theme, themesDir) {
  // Disable include resolution for now since the base themes work fine
  // and the include files have JSON parsing issues with control characters
  if (theme.include) {
    // Remove the include directive to avoid warnings
    const { include, ...themeWithoutInclude } = theme;
    return themeWithoutInclude;
  }
  return theme;
}

try {
  // Load custom themes
  const themesDir = path.join(projectRoot, 'themes');
  const darkThemePath = path.join(themesDir, 'dcbor-envelope-dark-color-theme.json');
  const lightThemePath = path.join(themesDir, 'dcbor-envelope-light-color-theme.json');

  // Read and parse theme files (handling JSONC)
  const darkThemeRaw = JSON.parse(stripJsonComments(fs.readFileSync(darkThemePath, 'utf8')));
  const lightThemeRaw = JSON.parse(stripJsonComments(fs.readFileSync(lightThemePath, 'utf8')));

  // Resolve includes
  const darkTheme = resolveThemeIncludes(darkThemeRaw, themesDir);
  const lightTheme = resolveThemeIncludes(lightThemeRaw, themesDir);

  // Load custom languages
  const dcborLanguagePath = path.join(projectRoot, 'syntaxes', 'dcbor-envelope.tmLanguage.json');
  const envelopePatternLanguagePath = path.join(projectRoot, 'syntaxes', 'patex.tmLanguage.json');

  const dcborLanguageGrammar = JSON.parse(fs.readFileSync(dcborLanguagePath, 'utf8'));
  const patexLanguageGrammar = JSON.parse(fs.readFileSync(envelopePatternLanguagePath, 'utf8'));

  // Create the language objects with proper structure
  const dcborLanguage = {
    id: 'dcbor-envelope',
    name: 'dcbor-envelope',
    scopeName: dcborLanguageGrammar.scopeName,
    ...dcborLanguageGrammar
  };

  const patexLanguage = {
    id: 'patex',
    name: 'patex',
    scopeName: patexLanguageGrammar.scopeName,
    ...patexLanguageGrammar
  };

  const highlighter = await createHighlighter({
    themes: [darkTheme, lightTheme],
    langs: [
      'javascript',
      'json',
      dcborLanguage,
      patexLanguage
    ]
  });

  // Map custom languages to the loaded language
  const langMap = {
    'envelope': 'dcbor-envelope',
    'dcbor': 'dcbor-envelope',
    'cbor': 'dcbor-envelope',
    'patex': 'Patex'
  };

  const actualLang = langMap[lang] || lang;

  const code = fs.readFileSync(0, 'utf8');

  if (isBatchMode) {
    // Batch processing mode for multiple inline snippets
    const snippets = JSON.parse(code);
    const results = {};

    for (const snippet of snippets) {
      const snippetLang = langMap[snippet.lang] || snippet.lang;
      const html = highlighter.codeToHtml(snippet.code, { lang: snippetLang, theme: lightTheme.name });

      // Extract just the inner content without the <pre> wrapper for inline use
      const match = html.match(/<code[^>]*>(.*?)<\/code>/s);
      if (match) {
        results[snippet.key] = `<code class="hljs dcbor-inline">${match[1]}</code>`;
      } else {
        results[snippet.key] = `<code class="hljs dcbor-inline">${snippet.code}</code>`;
      }
    }

    console.log(JSON.stringify(results));
  } else if (isInlineMode) {
    // For inline mode, generate simple HTML without theme switching
    const html = highlighter.codeToHtml(code, { lang: actualLang, theme: lightTheme.name });

    // Extract just the inner content without the <pre> wrapper for inline use
    const match = html.match(/<code[^>]*>(.*?)<\/code>/s);
    if (match) {
      console.log(`<code class="hljs dcbor-inline">${match[1]}</code>`);
    } else {
      console.log(`<code class="hljs dcbor-inline">${code}</code>`);
    }
  } else {
    // Generate HTML for both themes with proper CSS classes for mdbook theme switching
    const darkHtml = highlighter.codeToHtml(code, { lang: actualLang, theme: darkTheme.name });
    const lightHtml = highlighter.codeToHtml(code, { lang: actualLang, theme: lightTheme.name });

    // Wrap each theme in appropriate CSS classes that mdbook uses for theme switching
    const dualThemeHtml = `
<div class="light-theme-only">${lightHtml}</div>
<div class="dark-theme-only">${darkHtml}</div>`;

    console.log(dualThemeHtml);
  }
} catch (error) {
  console.error(`Error during initialization: ${error.message}`);
  console.error(`Stack: ${error.stack}`);
  process.exit(1);
}
