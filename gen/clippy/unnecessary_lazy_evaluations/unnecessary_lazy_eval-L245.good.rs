fn issue11672() {
    let _ = true.then_some::<&[u8]>({ &[] });
}
