fn regression_13524(a: usize, b: usize, c: bool) -> usize {
    if c {
        123
    } else if a >= b {
        //~^ implicit_saturating_sub
        0
    } else {
        b - a
    }
}
