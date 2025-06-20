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

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
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
    // ```lang\n…\n```
    let fence_re = Regex::new(r"(?s)```([a-zA-Z0-9_+-]+)\n(.*?)\n```")?;
    let mut out = String::with_capacity(md.len() + md.len() / 4);
    let mut last = 0;
    for cap in fence_re.captures_iter(md) {
        let m = cap.get(0).unwrap();
        out.push_str(&md[last..m.start()]);
        let lang = &cap[1];
        let code = &cap[2];
        if matches!(lang, "envelope" | "dcbor" | "cbor") {
            out.push_str(&shiki_html(code, lang)?);
        } else {
            out.push_str(m.as_str());
        }
        last = m.end();
    }
    out.push_str(&md[last..]);
    Ok(out)
}

fn shiki_html(code: &str, lang: &str) -> StdResult<String> {
    let mut child = Command::new("node")
        .arg("scripts/highlight.mjs")
        .arg(lang)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Cannot spawn Node (is it installed and on PATH?)")?;
    child.stdin.as_mut().unwrap().write_all(code.as_bytes())?;
    let output = child.wait_with_output()?;
    if !output.status.success() {
        anyhow::bail!("Shiki exited with error: {}", output.status);
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn main() {
    // Compatibility shim with mdBook's protocol.
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.first().map(|s| s.as_str()) == Some("supports") {
        // We claim to support all renderers (HTML/epub/etc.). Exit 0 for support.
        std::process::exit(0);
    }

    // Read the context & book from stdin as JSON.
    let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(io::stdin())
        .expect("Failed to parse mdbook input");

    let processed = DcborShiki
        .run(&ctx, book)
        .expect("Pre‑processor execution failed");

    // Write the processed book back to mdBook.
    serde_json::to_writer(io::stdout(), &processed).expect("Failed to write processed book to stdout");
}
