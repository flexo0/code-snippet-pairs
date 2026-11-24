fn unnecessary_sort_by() {
    vec.sort_by(|a, b| (***a).abs().cmp(&(***b).abs()));
}
