fn issue_8920() {
    let _ = vec
        .iter()
        .find(|f| f.ref_field.is_some())
        //~^ manual_find_map
        .map(|f| f.ref_field.cloned().unwrap());
}
