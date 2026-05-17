# clipfix

> Fix copy-pasted text from LLMs and the web — instantly convert fancy Unicode punctuation to plain ASCII.

## Installation

```bash
npm install -g clipfix
```

**macOS only** (ARM64 and Intel)

## Two Modes

**Soft mode (default)** — removes only invisible/structural characters (zero-width spaces, BOM, directional marks). Safe for prose, email, and multilingual text. Em dashes, curly quotes, and arrows are preserved.

**Hard mode (`--hard`)** — also replaces all typographic Unicode with ASCII equivalents. Use for terminal output, source code, or strictly ASCII pipelines.

## Usage

### Clipboard Mode
Copy text from anywhere, then:
```bash
clipfix --clipboard          # soft mode (default)
clipfix --hard --clipboard   # hard mode
# Fixed N characters
```

### Pipe Mode
```bash
# Soft sanitize a file (safe for prose and emails)
cat email.md | clipfix > email-clean.md

# Hard sanitize — flatten all Unicode punctuation to ASCII
echo '"Hello — world"' | clipfix --hard
# Output: "Hello -- world"

# Fix a file
cat document.md | clipfix --hard > fixed.md
```

### List All Replaced Characters
```bash
clipfix --list-replacements
```
Prints a full table of every character clipfix handles, grouped by soft/hard mode.

## Command Reference

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

### Soft mode (default)

| Unicode | Becomes | Description |
|---------|---------|-------------|
| U+00A0 | ` ` (space) | Non-breaking space |
| U+200B | *(removed)* | Zero-width space |
| U+FEFF | *(removed)* | BOM marker |
| U+200E/F | *(removed)* | Directional marks |

### Hard mode only

| Unicode | ASCII | Example |
|---------|-------|---------|
| `—` (em dash) | `--` | `Hello—world` → `Hello--world` |
| `""` (smart quotes) | `""` | `"hello"` → `"hello"` |
| `→` (arrow) | `->` | `step → next` → `step -> next` |
| `≠` | `!=` | `x ≠ y` → `x != y` |
| `…` | `...` | `wait…` → `wait...` |

Plus 15+ more — run `clipfix --list-replacements` for the full list.

## Why?

LLMs love fancy punctuation that breaks in terminals, code editors, and config files. clipfix sanitizes it instantly — with a safe default that won't mangle your multilingual email copy.

## License

MIT
