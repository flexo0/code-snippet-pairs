fn option_fold() {
    let _ = opt.as_ref().map_or(10, |x| 10 + x);
}
