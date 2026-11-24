fn unnecessary_sort_by() {
    vec.sort_by_key(|a| (a + 5).abs());
}
