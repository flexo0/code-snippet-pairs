fn issue_11524_2() -> Vec<i32> {
    vec.sort_by(|a, b| (b + 1).cmp(&(a + 1)));
}
