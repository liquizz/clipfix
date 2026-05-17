# clipfix

> Fix copy-pasted text from LLMs and the web — instantly convert fancy Unicode punctuation to plain ASCII.

## The Problem

You copy text from ChatGPT, Claude, or a webpage, paste it into your terminal or code editor, and... it breaks. Or looks weird. Or your linter complains.

**LLMs love fancy punctuation:**
- Em dashes (`—`) instead of double hyphens (`--`)
- Curly quotes (`""`) instead of straight quotes (`""`)
- Unicode arrows (`→`) instead of ASCII arrows (`->`)
- Zero-width spaces that cause cryptic errors

These characters look great in a browser but cause friction in:
- Terminals and shell scripts
- Source code and configuration files  
- Plain-text documentation
- Database queries and logs

## The Solution

`clipfix` sits between your clipboard and your workflow. One command, instant sanitization.

```bash
# Fix your clipboard contents instantly
clipfix --clipboard
# → "Fixed 12 characters"

# Or pipe text through it
echo "The answer—according to experts—is "yes"." | clipfix
# → The answer--according to experts--is "yes".
```

## Why clipfix?

- **Zero-config**: Install and use immediately
- **Blazing fast**: Rust-powered, handles huge files instantly
- **Clipboard-native**: `--clipboard` flag for copy-paste workflows
- **Unix-friendly**: Pipes, redirects, stdin/stdout — it just works
- **Invisible character killer**: Removes zero-width spaces, BOMs, and other invisible Unicode gotchas

## Installation

### macOS (Homebrew coming soon)
```bash
npm install -g clipfix
```

### From source (requires Rust)
```bash
git clone https://github.com/liquizz/clipfix
cd clipfix
cargo install --path .
```

## Usage

### Clipboard Mode (Easiest)
Copy text from anywhere, then:
```bash
clipfix --clipboard
# Fixed 8 characters
```
Your clipboard now contains sanitized text. Paste away.

### Pipe Mode (Power User)
```bash
# Fix a file
cat document.md | clipfix > fixed.md

# Fix LLM output directly
chatgpt "explain quantum computing" | clipfix

# Chain with other tools
echo "Error — code ≠ 0" | clipfix | grep "code != 0"
```

### Command Reference
```
clipfix [OPTIONS]

Options:
  -c, --clipboard  Read from and write to clipboard
  -h, --help       Print help
  -V, --version    Print version
```

## What Gets Fixed

| Character | Unicode | Becomes | When You'll Hit This |
|-----------|---------|---------|---------------------|
| `—` | U+2014 | `--` | LLM-generated prose with em dashes |
| `–` | U+2013 | `-` | Date ranges like 2020–2024 |
| `…` | U+2026 | `...` | Trailing thoughts... |
| `"` `"` | U+201C/U+201D | `"` | Copied quotes from web articles |
| `'` `'` | U+2018/U+2019 | `'` | Contractions and possessives |
| `→` | U+2192 | `->` | Documentation flow diagrams |
| `←` | U+2190 | `<-` | Arrow functions, assignment |
| `⇒` | U+21D2 | `=>` | Logic symbols, arrows |
| `×` | U+00D7 | `*` | Math operations |
| `≠` | U+2260 | `!=` | Code comparisons |
| `≤` `≥` | U+2264/U+2265 | `<=` `>=` | Mathematical comparisons |
| `•` | U+2022 | `*` | Bullet points |
| ` ` | U+00A0 | ` ` | Non-breaking spaces (invisible!) |
| `​` | U+200B | *(removed)* | Zero-width spaces (breaks everything!) |
| `﻿` | U+FEFF | *(removed)* | BOM markers |
| `←` `→` | U+2190/U+2192 | `<-` `->` | Arrows in documentation |

**Plus 10+ more** — see full list with `clipfix --help`

## Real-World Examples

### Before/After: LLM Chat Output
```
Input:  According to the docs—when using React's useEffect—you'll need…
Output: According to the docs--when using React's useEffect--you'll need...

Input:  The error "undefined" occurred in module "utils".
Output: The error "undefined" occurred in module "utils".
```

### Before/After: Technical Documentation
```
Input:  Run: git push → GitHub → CI/CD → Production
Output: Run: git push -> GitHub -> CI/CD -> Production

Input:  If x ≠ y and x ≤ 100, then x × 2 ≥ 0
Output: If x != y and x <= 100, then x * 2 >= 0
```

### Before/After: Invisible Character Nightmare
```
Input:  Some​text​with​zero​width​spaces  (looks normal!)
Output: Sometextwithzerowidthspaces     (actually clean)
```

## Pro Tips

**1. Alias for quick access**
```bash
alias cf='clipfix --clipboard'
# Now just type `cf` after copying anything
```

**2. Editor integration**
```vim
" In .vimrc — fix selected text
:'<,'>!clipfix
```

**3. Git pre-commit hook**
```bash
# Reject commits with fancy punctuation
git diff --cached | clipfix | diff - <(git diff --cached)
```

## License

MIT — do whatever you want with it.

---

Made with ⚡ by someone tired of manually fixing LLM output.
