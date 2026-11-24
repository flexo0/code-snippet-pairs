fn issue_8920() {
    let _ = vec
        .iter()
        .filter(|f| f.ref_field.is_some())
        //~^ manual_filter_map
        .map(|f| f.ref_field.cloned().unwrap());
}
