fn issue_8920() {
    let _ = vec
        .iter()
        .filter(|f| f.result_field.is_ok())
        //~^ manual_filter_map
        .map(|f| f.result_field.clone().unwrap());
}
