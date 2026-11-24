fn issue16581() {
    let _ = (1..=3).fold(0, |a, b| a + b);
}
