fn chars_cmp_with_unwrap() {
    if s.chars().last().unwrap() != '\n' {
        //~^ chars_last_cmp
        // !s.ends_with('o')
        // Nothing here
    }
}
