fn issue15269(a: usize, b: usize, c: usize) -> bool {
    a < b
        && match b {
            //~^ match_single_binding
            b => b < c,
        };
}
