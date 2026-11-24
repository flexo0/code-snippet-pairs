fn unnecessary_fold() {
    let _ = (0..3).fold(false, is_any);
}
