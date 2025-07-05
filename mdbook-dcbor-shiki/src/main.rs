use anyhow::{Context, Result};
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use regex::Regex;
use std::io::{self, Write};
use std::process::{Command, Stdio};

struct DcborShiki;

impl Preprocessor for DcborShiki {
    fn name(&self) -> &str {
        "dcbor-shiki"
    }

    fn run(
        &self,
        _ctx: &PreprocessorContext,
        mut book: Book,
    ) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ref mut ch) = *item {
                if let Ok(new) = highlight_chapter(&ch.content) {
                    ch.content = new;
                }
            }
        });
        Ok(book)
    }
}

type StdResult<T> = std::result::Result<T, anyhow::Error>;

fn highlight_chapter(md: &str) -> StdResult<String> {
    // Process inline syntax to avoid conflicts with mdbook-inline-highlighting
    let md_with_inline = process_inline_syntax(md)?;

    // Then process code blocks
    // ```lang\n…\n```
    let fence_re = Regex::new(r"(?s)```([a-zA-Z0-9_+-]+)\n(.*?)\n```")?;
    let mut out =
        String::with_capacity(md_with_inline.len() + md_with_inline.len() / 4);
    let mut last = 0;
    for cap in fence_re.captures_iter(&md_with_inline) {
        let m = cap.get(0).unwrap();
        out.push_str(&md_with_inline[last..m.start()]);
        let lang = &cap[1];
        let code = &cap[2];
        if matches!(lang, "envelope" | "dcbor" | "cbor" | "patex") {
            out.push_str(&shiki_html(code, lang)?);
        } else {
            out.push_str(m.as_str());
        }
        last = m.end();
    }
    out.push_str(&md_with_inline[last..]);
    Ok(out)
}

fn process_inline_syntax(md: &str) -> StdResult<String> {
    // Pattern for our custom inline syntax: `[lang] code`
    // Very specific pattern to avoid matching markdown links
    // Allow brackets in the code content but ensure we don't match nested backticks
    let inline_re = Regex::new(r"`\[(envelope|dcbor|cbor|patex)\]\s+([^`]+)`")?;

    let matches: Vec<_> = inline_re.captures_iter(md).collect();

    // If no matches, return original
    if matches.is_empty() {
        return Ok(md.to_string());
    }

    // Collect all unique code snippets for batch processing
    let mut unique_snippets = std::collections::HashMap::new();
    let mut snippet_id = 0u32;

    for caps in &matches {
        let lang = &caps[1];
        let code = &caps[2];

        let key = format!("{}:{}", lang, code);

        if !unique_snippets.contains_key(&key) {
            unique_snippets
                .insert(key, (snippet_id, lang.to_string(), code.to_string()));
            snippet_id += 1;
        }
    }

    // Batch process all snippets at once
    let highlighted_results = if !unique_snippets.is_empty() {
        match shiki_batch_inline_html(&unique_snippets) {
            Ok(results) => results,
            Err(_e) => {
                // Fallback to simple code tags for all
                unique_snippets.iter().map(|(key, (_id, lang, code))| {
                    (key.clone(), format!("<code class=\"dcbor-inline {} hljs\">{}</code>", lang, code))
                }).collect()
            }
        }
    } else {
        std::collections::HashMap::new()
    };

    let result = inline_re.replace_all(md, |caps: &regex::Captures| {
        let lang = &caps[1];
        let code = &caps[2];

        let key = format!("{}:{}", lang, code);

        let html =
            highlighted_results.get(&key).cloned().unwrap_or_else(|| {
                format!(
                    "<code class=\"dcbor-inline {} hljs\">{}</code>",
                    lang, code
                )
            });

        // Escape square brackets in the HTML to prevent link checker false positives
        // Only escape brackets that are not part of HTML tags
        escape_brackets_in_html(&html)
    });

    Ok(result.to_string())
}

/// Extracts content from HEREDOC assignment patterns in patex blocks.
///
/// Looks for patterns like:
/// ```
/// PATTERN=$(cat <<'EOF'
/// "hello"
/// EOF
/// )
/// ```
///
/// And returns the variable name and heredoc content: `("PATTERN", "\"hello\"")`
fn extract_heredoc_content(code: &str) -> Option<(String, String)> {
    let lines: Vec<&str> = code.lines().collect();

    // Look for the heredoc pattern
    let mut heredoc_start = None;
    let mut heredoc_end = None;
    let mut delimiter = None;
    let mut variable_name = None;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Look for HEREDOC start pattern: VARIABLE=$(cat <<'DELIMITER'
        if let Some(captures) =
            Regex::new(r"^(\w+)\s*=\s*\$\(cat\s+<<'([^']+)'\s*$")
                .ok()?
                .captures(trimmed)
        {
            variable_name = Some(captures.get(1)?.as_str());
            delimiter = Some(captures.get(2)?.as_str());
            heredoc_start = Some(i + 1);
            continue;
        }

        // Look for HEREDOC end pattern
        if let (Some(delim), Some(_)) = (delimiter, heredoc_start) {
            if trimmed == delim {
                heredoc_end = Some(i);
                break;
            }
        }
    }

    // Extract the content between start and end
    if let (Some(start), Some(end), Some(var_name)) =
        (heredoc_start, heredoc_end, variable_name)
    {
        if start < end {
            let content_lines = &lines[start..end];
            let content = content_lines.join("\n");
            return Some((var_name.to_string(), content.trim().to_string()));
        }
    }

    None
}

fn shiki_html(code: &str, lang: &str) -> StdResult<String> {
    // Special handling for patex blocks with HEREDOC assignments
    let processed_code = if lang == "patex" {
        if let Some((var_name, content)) = extract_heredoc_content(code) {
            format!("{}=\n{}", var_name, content)
        } else {
            code.to_string()
        }
    } else {
        code.to_string()
    };

    let mut child = Command::new("node")
        .arg("scripts/highlight.mjs")
        .arg(lang)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Cannot spawn Node (is it installed and on PATH?)")?;
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(processed_code.as_bytes())?;
    let output = child.wait_with_output()?;
    if !output.status.success() {
        anyhow::bail!("Shiki exited with error: {}", output.status);
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn shiki_batch_inline_html(
    snippets: &std::collections::HashMap<String, (u32, String, String)>,
) -> StdResult<std::collections::HashMap<String, String>> {
    // Create JSON input for batch processing
    let batch_input: Vec<serde_json::Value> = snippets
        .iter()
        .map(|(key, (id, lang, code))| {
            serde_json::json!({
                "id": id,
                "lang": lang,
                "code": code,
                "key": key
            })
        })
        .collect();

    let json_input = serde_json::to_string(&batch_input)?;

    let mut child = Command::new("node")
        .arg("scripts/highlight.mjs")
        .arg("--batch-inline")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Cannot spawn Node (is it installed and on PATH?)")?;

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(json_input.as_bytes())?;
    let output = child.wait_with_output()?;

    if !output.status.success() {
        anyhow::bail!("Shiki exited with error: {}", output.status);
    }

    let output_str = String::from_utf8(output.stdout)?;
    let results: std::collections::HashMap<String, String> =
        serde_json::from_str(&output_str)?;

    Ok(results)
}

fn escape_brackets_in_html(html: &str) -> String {
    // Replace square brackets and underscores with HTML entities
    // Square brackets to prevent link checker false positives
    // Underscores to prevent markdown italic processing
    // We need to be careful not to break HTML tags, so we only escape characters in text content
    let mut result = String::with_capacity(html.len() + html.len() / 4);
    let mut in_tag = false;
    let mut chars = html.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '<' => {
                in_tag = true;
                result.push(ch);
            }
            '>' => {
                in_tag = false;
                result.push(ch);
            }
            '[' if !in_tag => {
                result.push_str("&#91;");
            }
            ']' if !in_tag => {
                result.push_str("&#93;");
            }
            '_' if !in_tag => {
                result.push_str("&#95;");
            }
            _ => {
                result.push(ch);
            }
        }
    }

    result
}

fn main() {
    // Compatibility shim with mdBook's protocol.
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.first().map(|s| s.as_str()) == Some("supports") {
        // We claim to support all renderers (HTML/epub/etc.). Exit 0 for support.
        std::process::exit(0);
    }

    // Read the context & book from stdin as JSON.
    let (ctx, book) =
        mdbook::preprocess::CmdPreprocessor::parse_input(io::stdin())
            .expect("Failed to parse mdbook input");

    let processed = DcborShiki
        .run(&ctx, book)
        .expect("Pre‑processor execution failed");

    // Write the processed book back to mdBook.
    serde_json::to_writer(io::stdout(), &processed)
        .expect("Failed to write processed book to stdout");
}
