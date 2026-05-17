/// Sanitization mode controlling which Unicode characters are replaced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    /// Remove invisible/structural characters and AI-signature typographic characters
    /// (em dashes, curly quotes, ellipsis). Arrows, math symbols, and guillemets
    /// are left untouched.
    #[default]
    Soft,

    /// Replace all known Unicode punctuation with ASCII equivalents.
    /// Same behaviour as clipfix before v0.2.
    Hard,
}

/// A single character replacement entry.
#[derive(Debug)]
pub struct Replacement {
    /// The Unicode character to replace.
    pub from: char,
    /// Human-readable description of the character.
    pub description: &'static str,
    /// The ASCII string to substitute (empty string = remove the character).
    pub to: &'static str,
}

/// **Soft mode** replacements — applied in *both* soft and hard modes.
///
/// Includes invisible/structural characters and AI-signature typographic characters
/// (curly quotes, em/en dashes, ellipsis) that signal machine-generated content.
pub const SOFT_REPLACEMENTS: &[Replacement] = &[
    Replacement {
        from: '\u{00A0}',
        description: "Non-breaking space",
        to: " ",
    },
    Replacement {
        from: '\u{200B}',
        description: "Zero-width space",
        to: "",
    },
    Replacement {
        from: '\u{200C}',
        description: "Zero-width non-joiner",
        to: "",
    },
    Replacement {
        from: '\u{200D}',
        description: "Zero-width joiner",
        to: "",
    },
    Replacement {
        from: '\u{FEFF}',
        description: "BOM / zero-width no-break space",
        to: "",
    },
    Replacement {
        from: '\u{200E}',
        description: "Left-to-right mark",
        to: "",
    },
    Replacement {
        from: '\u{200F}',
        description: "Right-to-left mark",
        to: "",
    },
    Replacement {
        from: '\u{2018}',
        description: "Left single quotation mark",
        to: "'",
    },
    Replacement {
        from: '\u{2019}',
        description: "Right single quotation mark",
        to: "'",
    },
    Replacement {
        from: '\u{201C}',
        description: "Left double quotation mark",
        to: "\"",
    },
    Replacement {
        from: '\u{201D}',
        description: "Right double quotation mark",
        to: "\"",
    },
    Replacement {
        from: '\u{2014}',
        description: "Em dash",
        to: "--",
    },
    Replacement {
        from: '\u{2013}',
        description: "En dash",
        to: "-",
    },
    Replacement {
        from: '\u{2026}',
        description: "Horizontal ellipsis",
        to: "...",
    },
];

/// **Hard-only** replacements — applied exclusively in hard mode.
///
/// Visible typographic characters not covered by soft mode: arrows, guillemets,
/// math symbols, and prime marks.
pub const HARD_ONLY_REPLACEMENTS: &[Replacement] = &[
    Replacement {
        from: '\u{2022}',
        description: "Bullet",
        to: "*",
    },
    Replacement {
        from: '\u{2192}',
        description: "Rightwards arrow",
        to: "->",
    },
    Replacement {
        from: '\u{2190}',
        description: "Leftwards arrow",
        to: "<-",
    },
    Replacement {
        from: '\u{21D2}',
        description: "Rightwards double arrow",
        to: "=>",
    },
    Replacement {
        from: '\u{21D0}',
        description: "Leftwards double arrow",
        to: "<=",
    },
    Replacement {
        from: '\u{00AB}',
        description: "Left-pointing double angle quotation mark",
        to: "\"",
    },
    Replacement {
        from: '\u{00BB}',
        description: "Right-pointing double angle quotation mark",
        to: "\"",
    },
    Replacement {
        from: '\u{2032}',
        description: "Prime",
        to: "'",
    },
    Replacement {
        from: '\u{2033}',
        description: "Double prime",
        to: "\"",
    },
    Replacement {
        from: '\u{00D7}',
        description: "Multiplication sign",
        to: "*",
    },
    Replacement {
        from: '\u{00F7}',
        description: "Division sign",
        to: "/",
    },
    Replacement {
        from: '\u{00B1}',
        description: "Plus-minus sign",
        to: "+/-",
    },
    Replacement {
        from: '\u{2260}',
        description: "Not equal to",
        to: "!=",
    },
    Replacement {
        from: '\u{2264}',
        description: "Less-than or equal to",
        to: "<=",
    },
    Replacement {
        from: '\u{2265}',
        description: "Greater-than or equal to",
        to: ">=",
    },
    Replacement {
        from: '\u{2015}',
        description: "Horizontal bar",
        to: "--",
    },
];

/// Sanitize `input` according to the given [`Mode`].
///
/// - [`Mode::Soft`] — removes invisible / structural Unicode only.
/// - [`Mode::Hard`] — additionally replaces typographic characters with ASCII equivalents.
pub fn fix_text(input: &str, mode: Mode) -> String {
    let mut result = String::with_capacity(input.len());
    'char_loop: for ch in input.chars() {
        for entry in SOFT_REPLACEMENTS {
            if entry.from == ch {
                result.push_str(entry.to);
                continue 'char_loop;
            }
        }
        if mode == Mode::Hard {
            for entry in HARD_ONLY_REPLACEMENTS {
                if entry.from == ch {
                    result.push_str(entry.to);
                    continue 'char_loop;
                }
            }
        }
        result.push(ch);
    }
    result
}

/// Backward-compatible alias: hard-sanitize `input`.
///
/// Equivalent to `fix_text(input, Mode::Hard)`.
pub fn fix_punctuation(input: &str) -> String {
    fix_text(input, Mode::Hard)
}

/// Return a formatted table of all characters replaced by clipfix, grouped by mode.
pub fn format_replacement_list() -> String {
    fn render_section(replacements: &[Replacement]) -> String {
        let max_len = replacements
            .iter()
            .map(|r| r.description.len())
            .max()
            .unwrap_or(0);
        let mut s = String::new();
        for entry in replacements {
            let to_display = if entry.to.is_empty() {
                "(removed)".to_string()
            } else if entry.to == " " {
                "\" \" (regular space)".to_string()
            } else {
                entry.to.to_string()
            };
            s.push_str(&format!(
                "  U+{:04X}  {:<width$}  \u{2192}  {}\n",
                entry.from as u32,
                entry.description,
                to_display,
                width = max_len,
            ));
        }
        s
    }

    let mut out = String::new();
    out.push_str("Soft mode replacements (default \u{2014} applied in all modes):\n");
    out.push_str(&render_section(SOFT_REPLACEMENTS));
    out.push('\n');
    out.push_str("Hard mode replacements (applied with --hard / -H only):\n");
    out.push_str(&render_section(HARD_ONLY_REPLACEMENTS));
    out
}

#[cfg(test)]
mod tests {
    use super::{fix_punctuation, fix_text, Mode};

    #[test]
    fn soft_removes_non_breaking_space() {
        assert_eq!(fix_text("\u{00A0}", Mode::Soft), " ");
    }

    #[test]
    fn soft_removes_zero_width_space() {
        assert_eq!(fix_text("\u{200B}", Mode::Soft), "");
    }

    #[test]
    fn soft_removes_zero_width_non_joiner() {
        assert_eq!(fix_text("\u{200C}", Mode::Soft), "");
    }

    #[test]
    fn soft_removes_zero_width_joiner() {
        assert_eq!(fix_text("\u{200D}", Mode::Soft), "");
    }

    #[test]
    fn soft_removes_bom() {
        assert_eq!(fix_text("\u{FEFF}", Mode::Soft), "");
    }

    #[test]
    fn soft_removes_ltr_mark() {
        assert_eq!(fix_text("\u{200E}", Mode::Soft), "");
    }

    #[test]
    fn soft_removes_rtl_mark() {
        assert_eq!(fix_text("\u{200F}", Mode::Soft), "");
    }

    #[test]
    fn soft_replaces_em_dash() {
        assert_eq!(fix_text("\u{2014}", Mode::Soft), "--");
    }

    #[test]
    fn soft_replaces_curly_quotes() {
        assert_eq!(
            fix_text("\u{201C}hello\u{201D}", Mode::Soft),
            "\"hello\""
        );
    }

    #[test]
    fn soft_replaces_ellipsis() {
        assert_eq!(fix_text("\u{2026}", Mode::Soft), "...");
    }

    #[test]
    fn soft_ascii_passthrough() {
        assert_eq!(fix_text("Hello world", Mode::Soft), "Hello world");
    }

    #[test]
    fn test_smart_single_quote_open() {
        assert_eq!(fix_punctuation("\u{2018}"), "'");
    }

    #[test]
    fn test_smart_single_quote_close() {
        assert_eq!(fix_punctuation("\u{2019}"), "'");
    }

    #[test]
    fn test_smart_double_quote_open() {
        assert_eq!(fix_punctuation("\u{201C}"), "\"");
    }

    #[test]
    fn test_smart_double_quote_close() {
        assert_eq!(fix_punctuation("\u{201D}"), "\"");
    }

    #[test]
    fn test_em_dash() {
        assert_eq!(fix_punctuation("\u{2014}"), "--");
    }

    #[test]
    fn test_en_dash() {
        assert_eq!(fix_punctuation("\u{2013}"), "-");
    }

    #[test]
    fn test_ellipsis() {
        assert_eq!(fix_punctuation("\u{2026}"), "...");
    }

    #[test]
    fn test_bullet() {
        assert_eq!(fix_punctuation("\u{2022}"), "*");
    }

    #[test]
    fn test_non_breaking_space() {
        assert_eq!(fix_punctuation("\u{00A0}"), " ");
    }

    #[test]
    fn test_tier_1_kitchen_sink() {
        assert_eq!(
            fix_punctuation(
                "\u{2018}\u{2019}\u{201C}\u{201D}\u{2014}\u{2013}\u{2026}\u{2022}\u{00A0}"
            ),
            "''\"\"---...* "
        );
    }

    #[test]
    fn test_ascii_passthrough() {
        assert_eq!(fix_punctuation("Hello world"), "Hello world");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(fix_punctuation(""), "");
    }

    #[test]
    fn test_right_arrow() {
        assert_eq!(fix_punctuation("\u{2192}"), "->");
    }

    #[test]
    fn test_left_arrow() {
        assert_eq!(fix_punctuation("\u{2190}"), "<-");
    }

    #[test]
    fn test_right_double_arrow() {
        assert_eq!(fix_punctuation("\u{21D2}"), "=>");
    }

    #[test]
    fn test_left_double_arrow() {
        assert_eq!(fix_punctuation("\u{21D0}"), "<=");
    }

    #[test]
    fn test_left_guillemet() {
        assert_eq!(fix_punctuation("\u{00AB}"), "\"");
    }

    #[test]
    fn test_right_guillemet() {
        assert_eq!(fix_punctuation("\u{00BB}"), "\"");
    }

    #[test]
    fn test_prime_mark() {
        assert_eq!(fix_punctuation("\u{2032}"), "'");
    }

    #[test]
    fn test_double_prime_mark() {
        assert_eq!(fix_punctuation("\u{2033}"), "\"");
    }

    #[test]
    fn test_zero_width_space() {
        assert_eq!(fix_punctuation("\u{200B}"), "");
    }

    #[test]
    fn test_zero_width_non_joiner() {
        assert_eq!(fix_punctuation("\u{200C}"), "");
    }

    #[test]
    fn test_zero_width_joiner() {
        assert_eq!(fix_punctuation("\u{200D}"), "");
    }

    #[test]
    fn test_bom_zero_width_no_break_space() {
        assert_eq!(fix_punctuation("\u{FEFF}"), "");
    }

    #[test]
    fn test_left_to_right_mark() {
        assert_eq!(fix_punctuation("\u{200E}"), "");
    }

    #[test]
    fn test_right_to_left_mark() {
        assert_eq!(fix_punctuation("\u{200F}"), "");
    }

    #[test]
    fn test_multiplication_sign() {
        assert_eq!(fix_punctuation("\u{00D7}"), "*");
    }

    #[test]
    fn test_division_sign() {
        assert_eq!(fix_punctuation("\u{00F7}"), "/");
    }

    #[test]
    fn test_plus_minus_sign() {
        assert_eq!(fix_punctuation("\u{00B1}"), "+/-");
    }

    #[test]
    fn test_not_equal_to() {
        assert_eq!(fix_punctuation("\u{2260}"), "!=");
    }

    #[test]
    fn test_less_than_or_equal_to() {
        assert_eq!(fix_punctuation("\u{2264}"), "<=");
    }

    #[test]
    fn test_greater_than_or_equal_to() {
        assert_eq!(fix_punctuation("\u{2265}"), ">=");
    }

    #[test]
    fn test_horizontal_bar() {
        assert_eq!(fix_punctuation("\u{2015}"), "--");
    }

    #[test]
    fn test_tier_2_kitchen_sink() {
        assert_eq!(
            fix_punctuation(
                "\u{2192}\u{2190}\u{00AB}\u{00BB}\u{2032}\u{2033}\u{200B}\u{200E}\u{00D7}\u{00F7}\u{00B1}\u{2260}\u{2264}\u{2265}\u{2015}"
            ),
            "-><-\"\"'\"*/+/-!=<=>=--"
        );
    }
}
