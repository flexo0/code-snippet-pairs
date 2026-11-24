fn issue_8920() {
    let _ = vec
        .iter()
        .filter_map(|f| f.result_field.clone().ok());
}
