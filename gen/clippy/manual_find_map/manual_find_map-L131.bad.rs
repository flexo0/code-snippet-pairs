fn issue_8920() {
    let _ = vec
        .iter()
        .find(|f| f.result_field.is_ok())
        //~^ manual_find_map
        .map(|f| f.result_field.clone().unwrap());
}
