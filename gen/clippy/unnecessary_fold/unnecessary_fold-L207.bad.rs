fn option_fold() {
    let _ = opt.iter().fold(10, |acc, x| acc + x);
}
