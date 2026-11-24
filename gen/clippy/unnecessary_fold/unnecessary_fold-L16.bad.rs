fn unnecessary_fold() {
    let _ = (0..3).fold(false, |acc, x| is_any(acc, x));
}
