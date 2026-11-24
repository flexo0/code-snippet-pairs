fn unnecessary_sort_by() {
    vec.sort_by(|a, b| (b + 5).abs().cmp(&(a + 5).abs()));
}
