fn check(x: Option<u32>) {
    _ = x.as_ref().map_or_else(|| &[42][..0], std::slice::from_ref);
}
