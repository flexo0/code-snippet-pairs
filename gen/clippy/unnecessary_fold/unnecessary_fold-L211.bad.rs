fn option_fold() {
    let _ = opt.into_iter().fold(10, |acc, x| acc * x);
}
