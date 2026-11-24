fn issue11672_as() {
    let _ = None.get_or_insert({ &[] } as &[u8]);
}
