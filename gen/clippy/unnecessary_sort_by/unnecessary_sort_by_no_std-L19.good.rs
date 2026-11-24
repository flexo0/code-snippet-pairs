fn issue_11524_2() -> Vec<i32> {
    vec.sort_by_key(|a| core::cmp::Reverse(a + 1));
}
