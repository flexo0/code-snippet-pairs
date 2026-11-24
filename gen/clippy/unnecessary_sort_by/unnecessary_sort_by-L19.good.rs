fn unnecessary_sort_by() {
    vec.sort_unstable_by_key(|a| id(-a));
}
