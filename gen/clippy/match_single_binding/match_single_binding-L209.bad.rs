fn issue_10447() -> usize {
    let in_expr = [
        match 1 {
            //~^ match_single_binding
            _ => (),
        },
        match 1 {
            //~^ match_single_binding
            _ => side_effects(),
        },
        match 1 {
            //~^ match_single_binding
            _ => println!("1"),
        },
    ];
}
