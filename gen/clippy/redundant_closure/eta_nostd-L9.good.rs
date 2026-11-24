fn issue_13895() {
    let _: Option<Vec<u8>> = true.then(alloc::vec::Vec::new);
}
