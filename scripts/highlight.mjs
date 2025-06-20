#!/usr/bin/env node
import { getHighlighter } from 'shiki';
import fs from 'fs';

const [ , , lang = 'envelope' ] = process.argv;

const highlighter = await getHighlighter({
  themes: [
    { id: 'dcbor-light', path: '../themes/dcbor-envelope-light-color-theme.json' },
    { id: 'dcbor-dark',  path: '../themes/dcbor-envelope-dark-color-theme.json'  }
  ],
  langs: [
    { id: 'envelope', scopeName: 'source.envelope', path: '../syntaxes/dcbor-envelope.tmLanguage.json' }
  ]
});

const code = fs.readFileSync(0, 'utf8');
const html = highlighter.codeToHtml(code, { lang, theme: 'dcbor-dark' });
// The produced <pre> embeds both light & dark CSS vars, so only one pass is needed.
console.log(html);
