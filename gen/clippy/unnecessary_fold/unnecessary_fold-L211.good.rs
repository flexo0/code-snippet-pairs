fn option_fold() {
    let _ = opt.map_or(10, |x| 10 * x);
}
