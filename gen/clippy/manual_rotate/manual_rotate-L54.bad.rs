fn issue13028() {
    let _ = (x << s) | (x >> (32 - s));
}
