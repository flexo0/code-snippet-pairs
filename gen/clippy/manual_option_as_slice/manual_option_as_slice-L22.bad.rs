fn check(x: Option<u32>) {
    _ = x.as_ref().map_or_else(Default::default, std::slice::from_ref);
}
