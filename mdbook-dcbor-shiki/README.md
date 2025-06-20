## 🚀 Installation / Usage

1. Copy assets

```bash
cp -R path/to/vscode-dcbor-envelope/syntaxes   cbor-book/
cp -R path/to/vscode-dcbor-envelope/themes     cbor-book/
```

2. Install the helper tools

```bash
cd cbor-book
npm install             # brings in Shiki
```

```bash
cd cbor-book/mdbook-dcbor-shiki
cargo install --path .  # builds the Rust pre‑processor
```

3. Enable the pre‑processor

In `cbor‑book/book.toml` add:

```toml
[preprocessor.dcbor-shiki]
```

4. Write fenced blocks

```envelope
"Alice" [
    "knows": "Bob"
]
```

5. Build the book

```bash
cd cbor-book
mdbook build
mdbook serve
```

Your CBOR and Envelope code now renders with the exact colors you see in VS Code (switches automatically between light & dark based on the user’s preference).



## 📝 Notes & troubleshooting

- The pre‑processor runs once at build time, so the final HTML contains fully in‑lined `<pre class="shiki">…</pre>` with CSS vars—no runtime JS required.

- If you tweak your VS Code themes or grammar, just copy the updated JSON files over and rebuild the book.

- Shiki requires Node 18 or newer.  If you prefer a pure‑Rust stack you can replace the Node helper with `shiki-rs`, but the JS path is the smallest diff.

- Feel free to rename the pre‑processor or move it into your workspace’s `tools/` folder; just keep the relative paths in `highlight.mjs` up to date.
