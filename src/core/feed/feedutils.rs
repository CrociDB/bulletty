#[must_use]
pub fn normalize_and_truncate<S: AsRef<str>>(input: S, max_len: usize) -> String {
    let s = input.as_ref();
    let mut out = String::with_capacity(s.len());
    let mut in_ws = false;

    for c in s.chars() {
        if c.is_whitespace() {
            if !in_ws {
                out.push(' ');
                in_ws = true;
            }
        } else {
            out.push(c);
            in_ws = false;
        }
    }

    let trimmed = out.trim();
    let char_count = trimmed.chars().count();

    if char_count <= max_len {
        trimmed.to_string()
    } else if max_len <= 3 {
        trimmed.chars().take(max_len).collect()
    } else {
        let truncated: String = trimmed.chars().take(max_len - 3).collect();
        format!("{truncated}...")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_unchanged_short_string() {
        assert_eq!(normalize_and_truncate("hello", 10), "hello");
    }

    #[test]
    fn collapses_multiple_spaces() {
        assert_eq!(normalize_and_truncate("hello    world", 50), "hello world");
    }

    #[test]
    fn collapses_various_whitespace() {
        assert_eq!(
            normalize_and_truncate("hello\t\n\r  world", 50),
            "hello world"
        );
    }

    #[test]
    fn trims_leading_and_trailing_whitespace() {
        assert_eq!(normalize_and_truncate("   hello   ", 50), "hello");
    }

    #[test]
    fn truncates_long_string_with_ellipsis() {
        assert_eq!(normalize_and_truncate("hello world", 8), "hello...");
    }

    #[test]
    fn truncates_exactly_at_max_len() {
        assert_eq!(normalize_and_truncate("hello", 5), "hello");
        assert_eq!(normalize_and_truncate("hello!", 5), "he...");
    }

    #[test]
    fn handles_empty_string() {
        assert_eq!(normalize_and_truncate("", 10), "");
    }

    #[test]
    fn handles_whitespace_only_string() {
        assert_eq!(normalize_and_truncate("   \t\n  ", 10), "");
    }

    #[test]
    fn handles_unicode_correctly() {
        // Each emoji is one char
        assert_eq!(normalize_and_truncate("🎉🎊🎁🎂🎈", 5), "🎉🎊🎁🎂🎈");
        assert_eq!(normalize_and_truncate("🎉🎊🎁🎂🎈🎆", 5), "🎉🎊...");
    }

    #[test]
    fn handles_mixed_unicode_and_ascii() {
        assert_eq!(normalize_and_truncate("Hello 世界!", 20), "Hello 世界!");
        assert_eq!(normalize_and_truncate("Hello 世界!", 8), "Hello...");
    }

    #[test]
    fn normalizes_then_truncates() {
        // Multiple spaces should be collapsed before truncation
        let input = "hello      world     test";
        // Normalized: "hello world test" (16 chars)
        assert_eq!(normalize_and_truncate(input, 20), "hello world test");
        assert_eq!(normalize_and_truncate(input, 10), "hello w...");
    }

    #[test]
    fn accepts_string_and_str() {
        let owned = String::from("test string");
        let borrowed = "test string";

        assert_eq!(normalize_and_truncate(owned, 50), "test string");
        assert_eq!(normalize_and_truncate(borrowed, 50), "test string");
    }

    #[test]
    fn handles_newlines_in_titles() {
        // Real-world case: RSS titles sometimes have embedded newlines
        let input = "Breaking News:\n  Something Happened";
        assert_eq!(
            normalize_and_truncate(input, 50),
            "Breaking News: Something Happened"
        );
    }

    #[test]
    fn handles_very_small_max_len() {
        // Edge case: max_len too small for ellipsis, just truncate
        assert_eq!(normalize_and_truncate("hello", 3), "hel");
        assert_eq!(normalize_and_truncate("hello", 2), "he");
        assert_eq!(normalize_and_truncate("hello", 1), "h");
        assert_eq!(normalize_and_truncate("hello", 0), "");
    }

    #[test]
    fn handles_string_exactly_at_boundary() {
        assert_eq!(normalize_and_truncate("1234567890", 10), "1234567890");
        assert_eq!(normalize_and_truncate("12345678901", 10), "1234567...");
    }
}
