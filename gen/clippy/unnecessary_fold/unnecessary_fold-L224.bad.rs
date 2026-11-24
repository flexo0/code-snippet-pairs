fn option_fold() {
    let _ = opt.iter().fold(2, |acc, x| acc * acc + x);
}
