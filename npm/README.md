# clipfix

> Fix copy-pasted text from LLMs and the web — instantly convert fancy Unicode punctuation to plain ASCII.

## Installation

```bash
npm install -g clipfix
```

**macOS only** (ARM64 and Intel)

## Usage

### Clipboard Mode
Copy text from anywhere, then:
```bash
clipfix --clipboard
# Fixed 8 characters
```

### Pipe Mode
```bash
echo '"Hello — world"' | clipfix
# Output: "Hello -- world"

# Fix a file
cat document.md | clipfix > fixed.md
```

## What Gets Fixed

| Unicode | ASCII | Example |
|---------|-------|---------|
| `—` (em dash) | `--` | `Hello—world` → `Hello--world` |
| `""` (smart quotes) | `""` | `"hello"` → `"hello"` |
| `→` (arrow) | `->` | `step → next` → `step -> next` |
| `≠` | `!=` | `x ≠ y` → `x != y` |
| `…` | `...` | `wait…` → `wait...` |

Plus 20+ more transformations including zero-width space removal.

## Why?

LLMs love fancy punctuation that breaks in terminals, code editors, and config files. clipfix sanitizes it instantly.

## License

MIT
