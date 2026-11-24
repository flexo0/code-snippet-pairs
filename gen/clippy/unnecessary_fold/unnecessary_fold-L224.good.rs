fn option_fold() {
    let _ = opt.as_ref().map_or(2, |x| 2 * 2 + x);
}
