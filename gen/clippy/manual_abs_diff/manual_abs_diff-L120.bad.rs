fn issue15254(a: &usize, b: &usize) -> usize {
    if a < b {
        //~^ manual_abs_diff
        b - a
    } else {
        a - b
    }
}
