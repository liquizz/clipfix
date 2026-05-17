pub const REPLACEMENTS: &[(char, &str)] = &[
    ('\u{2018}', "'"),
    ('\u{2019}', "'"),
    ('\u{201C}', "\""),
    ('\u{201D}', "\""),
    ('\u{2014}', "--"),
    ('\u{2013}', "-"),
    ('\u{2026}', "..."),
    ('\u{2022}', "*"),
    ('\u{00A0}', " "),
    ('\u{2192}', "->"),
    ('\u{2190}', "<-"),
    ('\u{21D2}', "=>"),
    ('\u{21D0}', "<="),
    ('\u{00AB}', "\""),
    ('\u{00BB}', "\""),
    ('\u{2032}', "'"),
    ('\u{2033}', "\""),
    ('\u{200B}', ""),
    ('\u{200C}', ""),
    ('\u{200D}', ""),
    ('\u{FEFF}', ""),
    ('\u{200E}', ""),
    ('\u{200F}', ""),
    ('\u{00D7}', "*"),
    ('\u{00F7}', "/"),
    ('\u{00B1}', "+/-"),
    ('\u{2260}', "!="),
    ('\u{2264}', "<="),
    ('\u{2265}', ">="),
    ('\u{2015}', "--"),
];

pub fn fix_punctuation(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for ch in input.chars() {
        if let Some((_, replacement)) = REPLACEMENTS.iter().find(|(from, _)| *from == ch) {
            result.push_str(replacement);
        } else {
            result.push(ch);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::fix_punctuation;

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
