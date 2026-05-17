# clipfix

> Fix copy-pasted text from LLMs and the web -- instantly convert fancy Unicode punctuation to plain ASCII.

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
# → "Fixed 0 characters"  (soft mode -- only invisible chars replaced)

# Or pipe text through it
echo "The answer—according to experts—is "yes"." | clipfix --hard
# → The answer--according to experts--is "yes".
```

## Two Sanitization Modes

clipfix ships with two modes to handle different use cases.

### Soft mode (default)

Removes invisible/structural characters **and** the typographic characters most commonly produced by AI text generators -- curly quotes (`""''`), em/en dashes (`---`), and ellipsis (`...`). Arrows, math symbols, and guillemets are left untouched.

```bash
echo "ospiti internazionali — da Israele" | clipfix
# → ospiti internazionali -- da Israele   (em dash replaced)

echo "He said "ciao" to everyone" | clipfix
# → He said "ciao" to everyone
```

### Hard mode (`--hard` / `-H`)

Replaces **all known Unicode punctuation** with ASCII equivalents -- everything soft mode does, plus arrows → `->`, math symbols, and more.

Use this when targeting:
- Terminal output or shell scripts
- Source code and config files
- Plain-text formats that must be ASCII-safe

```bash
echo "The answer—"yes"—according to experts." | clipfix --hard
# → The answer--"yes"--according to experts.
```

## Why clipfix?

- **Zero-config**: Install and use immediately
- **Blazing fast**: Rust-powered, handles huge files instantly
- **Safe default**: Soft mode protects natural-language text from over-sanitization
- **Clipboard-native**: `--clipboard` flag for copy-paste workflows
- **Unix-friendly**: Pipes, redirects, stdin/stdout -- it just works
- **Invisible character killer**: Removes zero-width spaces, BOMs, and other invisible Unicode gotchas

## Installation

### macOS (via npm)
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
# Fixed 0 characters  (soft mode -- only invisible chars replaced)

clipfix --hard --clipboard
# Fixed 8 characters  (hard mode -- all typographic chars replaced)
```
Your clipboard now contains sanitized text. Paste away.

### Pipe Mode (Power User)
```bash
# Soft sanitize a file
cat email.md | clipfix > email-clean.md

# Hard sanitize LLM output for terminal use
chatgpt "explain quantum computing" | clipfix --hard

# Chain with other tools
echo "Error -- code ≠ 0" | clipfix --hard | grep "code != 0"
```

### List All Replaced Characters
```bash
clipfix --list-replacements
```
Prints a full table of every character clipfix handles, grouped by mode. Useful for auditing what gets changed in your pipeline.

### Command Reference
```
clipfix [OPTIONS]

Options:
  -S, --soft               Soft sanitize: remove only invisible/structural characters (default)
  -H, --hard               Hard sanitize: also replace typographic characters with ASCII equivalents
  -c, --clipboard          Read from and write to clipboard
  -l, --list-replacements  List all characters replaced by clipfix, grouped by mode
  -h, --help               Print help
  -V, --version            Print version
```

## What Gets Fixed

### Soft mode (default) -- always applied

Removes invisible/structural characters **and** the typographic characters most commonly produced by AI text generators (curly quotes, em/en dashes, ellipsis).

| Character | Unicode | Becomes | Why |
|-----------|---------|---------|-----|
| ` ` | U+00A0 | ` ` (space) | Non-breaking spaces cause invisible layout bugs |
| `​` | U+200B | *(removed)* | Zero-width spaces break parsers silently |
| `‌` | U+200C | *(removed)* | Zero-width non-joiner |
| `‍` | U+200D | *(removed)* | Zero-width joiner |
| `﻿` | U+FEFF | *(removed)* | BOM markers corrupt file parsing |
| `‎` | U+200E | *(removed)* | Left-to-right mark |
| `‏` | U+200F | *(removed)* | Right-to-left mark |
| `'` `'` | U+2018/U+2019 | `'` | AI-generated curly single quotes |
| `"` `"` | U+201C/U+201D | `"` | AI-generated curly double quotes |
| `—` | U+2014 | `--` | AI-generated em dashes |
| `–` | U+2013 | `-` | AI-generated en dashes |
| `…` | U+2026 | `...` | AI-generated ellipsis |

### Hard mode only (`--hard`) — remaining typographic characters

| Character | Unicode | Becomes | When You'll Hit This |
|-----------|---------|---------|---------------------|
| `→` | U+2192 | `->` | Documentation flow diagrams |
| `←` | U+2190 | `<-` | Arrow functions, assignment |
| `⇒` | U+21D2 | `=>` | Logic symbols, arrows |
| `⇐` | U+21D0 | `<=` | Double arrow left |
| `«` `»` | U+00AB/U+00BB | `"` | French/Italian guillemets |
| `×` | U+00D7 | `*` | Math operations |
| `÷` | U+00F7 | `/` | Division |
| `±` | U+00B1 | `+/-` | Plus-minus |
| `≠` | U+2260 | `!=` | Code comparisons |
| `≤` `≥` | U+2264/U+2265 | `<=` `>=` | Mathematical comparisons |
| `•` | U+2022 | `*` | Bullet points |
| `′` `″` | U+2032/U+2033 | `'` `"` | Prime marks |
| `―` | U+2015 | `--` | Horizontal bar |

Run `clipfix --list-replacements` for the complete machine-readable list.

## Real-World Examples

### Email pipeline (soft mode -- default)
```
Input:  ospiti internazionali — da Israele, dagli Stati Uniti
Output: ospiti internazionali -- da Israele, dagli Stati Uniti
        (em dash replaced — soft mode removes AI-signature punctuation)
```

### LLM chat output (hard mode)
```
Input:  According to the docs—when using React's useEffect—you'll need…
Output: According to the docs--when using React's useEffect--you'll need...

Input:  The error "undefined" occurred in module "utils".
Output: The error "undefined" occurred in module "utils".
```

### Technical documentation (hard mode)
```
Input:  Run: git push → GitHub → CI/CD → Production
Output: Run: git push -> GitHub -> CI/CD -> Production

Input:  If x ≠ y and x ≤ 100, then x × 2 ≥ 0
Output: If x != y and x <= 100, then x * 2 >= 0
```

### Invisible character nightmare
```
Input:  Some​text​with​zero​width​spaces  (looks normal!)
Output: Sometextwithzerowidthspaces     (actually clean — soft mode handles this)
```

## Pro Tips

**1. Alias for quick access**
```bash
alias cf='clipfix --clipboard'
alias cfh='clipfix --hard --clipboard'
```

**2. Editor integration**
```vim
" Hard sanitize selected text (ASCII-only output)
:'<,'>!clipfix --hard
```

**3. Git pre-commit hook (hard mode)**
```bash
# Reject commits with fancy punctuation in source files
git diff --cached | clipfix --hard | diff - <(git diff --cached)
```

## License

MIT -- do whatever you want with it.

---

Made with ⚡ by someone tired of manually fixing LLM output.
