fn issue12758(s: &[u8]) {
    match &s[0..3] {
        b"foo" => println!(),
        _ => {},
    }
}
