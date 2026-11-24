fn issue12758(s: &[u8]) {
    if &s[0..3] == b"foo" { println!() }
}
