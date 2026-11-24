fn issue11672_as() {
    let _ = None.get_or_insert_with(|| -> &[u8] { &[] });
}
