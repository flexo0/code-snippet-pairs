fn regression_13524(a: usize, b: usize, c: bool) -> usize {
    if c {
        123
    } else { b.saturating_sub(a) }
}
