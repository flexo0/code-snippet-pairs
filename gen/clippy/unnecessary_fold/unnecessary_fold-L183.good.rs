fn issue16581() {
    let _ = (0..3).all(|x| x > 2);
}
