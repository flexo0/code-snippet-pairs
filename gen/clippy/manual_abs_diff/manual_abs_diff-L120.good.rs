fn issue15254(a: &usize, b: &usize) -> usize {
    b.abs_diff(*a)
}
