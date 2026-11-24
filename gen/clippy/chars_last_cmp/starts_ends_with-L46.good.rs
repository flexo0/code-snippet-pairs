fn chars_cmp_with_unwrap() {
    if !s.ends_with('\n') {
        //~^ chars_last_cmp
        // !s.ends_with('o')
        // Nothing here
    }
}
