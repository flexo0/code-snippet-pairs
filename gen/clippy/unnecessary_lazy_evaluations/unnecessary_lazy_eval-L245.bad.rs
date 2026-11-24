fn issue11672() {
    let _ = true.then(|| -> &[u8] { &[] });
}
