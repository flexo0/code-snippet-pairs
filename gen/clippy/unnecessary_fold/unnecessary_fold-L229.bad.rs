fn option_fold() {
    let _ = opt.iter().fold(init, |acc, x| acc + x);
}
