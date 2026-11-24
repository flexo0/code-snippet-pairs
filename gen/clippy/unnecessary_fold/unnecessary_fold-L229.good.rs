fn option_fold() {
    let _ = opt.as_ref().map_or(init, |x| init + x);
}
