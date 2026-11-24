fn check(x: Option<u32>) {
    _ = x.as_ref().map(std::slice::from_ref).unwrap_or_default();
}
