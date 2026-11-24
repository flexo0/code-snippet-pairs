fn issue_10447() -> usize {
    let in_expr = [
        (),
        side_effects(),
        println!("1"),
    ];
}
