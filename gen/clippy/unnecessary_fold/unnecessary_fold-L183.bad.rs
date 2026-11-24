fn issue16581() {
    let _ = (0..3).fold(true, |acc, x| x > 2 && acc);
}
