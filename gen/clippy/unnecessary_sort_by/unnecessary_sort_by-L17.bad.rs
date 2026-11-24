fn unnecessary_sort_by() {
    vec.sort_by(|a, b| (a + 5).abs().cmp(&(b + 5).abs()));
}
