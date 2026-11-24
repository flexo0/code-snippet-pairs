fn issue_10447() -> usize {
    match 1 {
        //~^ match_single_binding
        _ => (),
    }
}
