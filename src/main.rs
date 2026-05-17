use clap::Parser;
use clipfix::fix_punctuation;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(
    name = "clipfix",
    version,
    about = "Replace LLM special punctuation with ASCII equivalents"
)]
struct Cli {
    #[arg(short, long)]
    clipboard: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.clipboard {
        let mut clipboard = arboard::Clipboard::new().unwrap_or_else(|e| {
            eprintln!("Failed to open clipboard: {e}");
            std::process::exit(1);
        });

        let original = clipboard.get_text().unwrap_or_else(|e| {
            eprintln!("Failed to read clipboard: {e}");
            std::process::exit(1);
        });

        let fixed = fix_punctuation(&original);

        let changed_count = original
            .chars()
            .zip(fixed.chars())
            .filter(|(a, b)| a != b)
            .count();

        clipboard.set_text(fixed).unwrap_or_else(|e| {
            eprintln!("Failed to write clipboard: {e}");
            std::process::exit(1);
        });

        eprintln!("Fixed {changed_count} characters");
        return;
    }

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let output = fix_punctuation(&input);
    io::stdout()
        .write_all(output.as_bytes())
        .expect("Failed to write stdout");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clipboard_flag_parses_to_true() {
        let cli = Cli::try_parse_from(["clipfix", "--clipboard"]).unwrap();
        assert!(cli.clipboard);
    }

    #[test]
    fn no_clipboard_flag_defaults_false() {
        let cli = Cli::try_parse_from(["clipfix"]).unwrap();
        assert!(!cli.clipboard);
    }
}
