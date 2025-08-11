fn normalize(input: &str) -> String {
    // Remove BOM at the very start only
    let s = if let Some(rest) = input.strip_prefix('\u{FEFF}') {
        rest
    } else {
        input
    };

    // Normalize end-of-line to LF
    let s = s.replace("\r\n", "\n").replace('\r', "\n");

    let parts = s.split('\n');
    let trimmed: Vec<&str> = parts
        .map(|line| line.trim_end_matches([' ', '\t']))
        .collect();
    trimmed.join("\n")
}

/// Compares two strings while ignoring BOM, EOL differences, and trailing spaces/tabs at line ends.
/// Final newline presence is preserved by `normalize`, so differing final-newline status results in a diff.
pub(crate) fn string_diff(source: &str, dest: &str) -> bool {
    normalize(source) != normalize(dest)
}

#[cfg(test)]
mod tests {
    use super::{normalize, string_diff};

    #[test]
    fn normalize_removes_bom_converts_eol_and_trims_trailing_ws() {
        let input = "\u{FEFF}line1  \t\r\nline2\t \rline3 \n";
        let expected = "line1\nline2\nline3\n";
        assert_eq!(normalize(input), expected);
    }

    #[test]
    fn normalize_preserves_final_newline_presence() {
        assert_eq!(normalize("a\n"), "a\n");
        assert_eq!(normalize("a"), "a");
        assert!(string_diff("a\n", "a"));
    }

    #[test]
    fn string_diff_ignores_bom_eol_and_trailing_ws() {
        let source = "\u{FEFF}foo  \r\nbar\t \r\nbaz\t";
        let dest = "foo\nbar\nbaz";
        assert!(!string_diff(source, dest));
    }

    #[test]
    fn string_diff_detects_real_content_change() {
        assert!(string_diff("abc", "abd"));
        assert!(string_diff("x", ""));
        assert!(string_diff("a\nb", "a\nc"));
    }

    #[test]
    fn normalize_trailing_nbsp_is_preserved_but_spaces_tabs_trimmed() {
        let nbsp = '\u{00A0}';
        let input = format!("a{nbsp} \n");
        let expected = format!("a{nbsp}\n");
        assert_eq!(normalize(&input), expected);
    }

    #[test]
    fn normalize_preserves_leading_spaces_and_tabs() {
        let input = "  a\n\tb \n";
        let expected = "  a\n\tb\n";
        assert_eq!(normalize(input), expected);
    }

    #[test]
    fn normalize_removes_bom_only_at_start() {
        let input = "\u{FEFF}a\n\u{FEFF}b";
        let expected = "a\n\u{FEFF}b";
        assert_eq!(normalize(input), expected);
    }

    #[test]
    fn normalize_empty_string() {
        assert_eq!(normalize(""), "");
    }

    #[test]
    fn normalize_trims_mixed_trailing_tabs_and_spaces() {
        let input = "a\t \t\n";
        let expected = "a\n";
        assert_eq!(normalize(input), expected);
    }

    #[test]
    fn normalize_converts_cr_only_to_lf() {
        let input = "a\rb\rc";
        let expected = "a\nb\nc";
        assert_eq!(normalize(input), expected);
    }
}
