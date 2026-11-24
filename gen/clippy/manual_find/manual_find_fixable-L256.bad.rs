mod issue14826 {
    fn adjust_fixable(needle: &str) -> Option<&'static str> {
        for candidate in &["foo", "bar"] {
            //~^ manual_find
            if candidate.eq_ignore_ascii_case(needle) {
                return Some(candidate);
            }
        }
        None
    }
}
