fn issue_8920() {
    let _ = vec
        .iter()
        .find_map(|f| f.ref_field.cloned());
}
