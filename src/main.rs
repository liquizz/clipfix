use clap::Parser;
use clipfix::{fix_text, format_replacement_list, Mode};
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(
    name = "clipfix",
    version,
    about = "Replace Unicode punctuation with ASCII equivalents",
    long_about = "Replace Unicode punctuation with ASCII equivalents.\n\n\
                  Default mode (--soft) removes only invisible/structural characters,\n\
                  keeping typographic characters like em dashes and curly quotes intact.\n\
                  Use --hard to aggressively convert all typographic Unicode to ASCII.\n\n\
                  Run with --list-replacements to see every character that clipfix handles."
)]
struct Cli {
    #[arg(short = 'S', long, conflicts_with = "hard", help = "Soft sanitize: remove only invisible/structural characters (default)")]
    soft: bool,

    #[arg(short = 'H', long, conflicts_with = "soft", help = "Hard sanitize: also replace typographic characters with ASCII equivalents")]
    hard: bool,

    #[arg(short, long, help = "Read from and write to clipboard")]
    clipboard: bool,

    #[arg(short = 'l', long, help = "List all characters replaced by clipfix, grouped by mode")]
    list_replacements: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.list_replacements {
        print!("{}", format_replacement_list());
        return;
    }

    let mode = if cli.hard { Mode::Hard } else { Mode::Soft };

    if cli.clipboard {
        let mut clipboard = arboard::Clipboard::new().unwrap_or_else(|e| {
            eprintln!("Failed to open clipboard: {e}");
            std::process::exit(1);
        });

        let original = clipboard.get_text().unwrap_or_else(|e| {
            eprintln!("Failed to read clipboard: {e}");
            std::process::exit(1);
        });

        let fixed = fix_text(&original, mode);

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

    let output = fix_text(&input, mode);
    io::stdout()
        .write_all(output.as_bytes())
        .expect("Failed to write stdout");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soft_flag_parses() {
        let cli = Cli::try_parse_from(["clipfix", "--soft"]).unwrap();
        assert!(cli.soft);
        assert!(!cli.hard);
    }

    #[test]
    fn hard_flag_parses() {
        let cli = Cli::try_parse_from(["clipfix", "--hard"]).unwrap();
        assert!(cli.hard);
        assert!(!cli.soft);
    }

    #[test]
    fn soft_and_hard_conflict() {
        assert!(Cli::try_parse_from(["clipfix", "--soft", "--hard"]).is_err());
    }

    #[test]
    fn default_mode_is_soft() {
        let cli = Cli::try_parse_from(["clipfix"]).unwrap();
        assert!(!cli.hard);
    }

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

    #[test]
    fn list_replacements_flag_parses() {
        let cli = Cli::try_parse_from(["clipfix", "--list-replacements"]).unwrap();
        assert!(cli.list_replacements);
    }
}
