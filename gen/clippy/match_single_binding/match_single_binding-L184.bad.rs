fn issue_10447() -> usize {
    let a = match 1 {
        //~^ match_single_binding
        _ => (),
    };
}
