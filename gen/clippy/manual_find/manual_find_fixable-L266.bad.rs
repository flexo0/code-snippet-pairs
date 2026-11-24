mod issue14826 {
    fn adjust_unfixable(needle: &str) -> Option<*const str> {
        for &candidate in &["foo", "bar"] {
            //~^ manual_find
            if candidate.eq_ignore_ascii_case(needle) {
                return Some(candidate);
            }
        }
        None
    }
}
