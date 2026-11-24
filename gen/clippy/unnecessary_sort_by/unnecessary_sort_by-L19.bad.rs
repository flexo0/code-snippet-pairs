fn unnecessary_sort_by() {
    vec.sort_unstable_by(|a, b| id(-a).cmp(&id(-b)));
}
