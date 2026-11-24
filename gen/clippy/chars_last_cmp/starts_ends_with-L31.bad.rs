fn chars_cmp_with_unwrap() {
    if s.chars().last().unwrap() == 'o' {
        //~^ chars_last_cmp
        // s.ends_with('o')
        // Nothing here
    }
}
