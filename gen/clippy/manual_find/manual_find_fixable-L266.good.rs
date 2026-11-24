mod issue14826 {
    fn adjust_unfixable(needle: &str) -> Option<*const str> {
        ["foo", "bar"].iter().find(|&&candidate| candidate.eq_ignore_ascii_case(needle)).copied().map(|v| v as _)
    }
}
