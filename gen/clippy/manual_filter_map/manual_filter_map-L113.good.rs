fn issue_8920() {
    let _ = vec
        .iter()
        .filter_map(|f| f.ref_field.cloned());
}
