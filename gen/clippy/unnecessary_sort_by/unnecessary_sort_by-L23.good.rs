fn unnecessary_sort_by() {
    vec.sort_by_key(|a| std::cmp::Reverse((a + 5).abs()));
}
