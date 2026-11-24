mod issue14826 {
    fn adjust_fixable(needle: &str) -> Option<&'static str> {
        ["foo", "bar"].iter().find(|&candidate| candidate.eq_ignore_ascii_case(needle)).map(|v| v as _)
    }
}
