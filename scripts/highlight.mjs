#!/usr/bin/env node
import { createHighlighter } from 'shiki';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const projectRoot = path.dirname(__dirname);

const [, , lang = 'envelope'] = process.argv;

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

  // Load custom language
  const languagePath = path.join(projectRoot, 'syntaxes', 'dcbor-envelope.tmLanguage.json');
  const customLanguageGrammar = JSON.parse(fs.readFileSync(languagePath, 'utf8'));

  // Create the language object with proper structure
  const customLanguage = {
    name: 'dcbor-envelope',
    scopeName: customLanguageGrammar.scopeName,
    ...customLanguageGrammar
  };

  const highlighter = await createHighlighter({
    themes: [darkTheme, lightTheme],
    langs: [
      'javascript',
      'json',
      customLanguage
    ]
  });

  // Map custom languages to the loaded language
  const langMap = {
    'envelope': 'dcbor-envelope',
    'dcbor': 'dcbor-envelope',
    'cbor': 'dcbor-envelope'
  };

  const actualLang = langMap[lang] || lang;

  const code = fs.readFileSync(0, 'utf8');
  const html = highlighter.codeToHtml(code, { lang: actualLang, theme: darkTheme.name });
  console.log(html);
} catch (error) {
  console.error(`Error during initialization: ${error.message}`);
  console.error(`Stack: ${error.stack}`);
  process.exit(1);
}
