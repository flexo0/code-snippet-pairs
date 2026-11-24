fn issue16894() {
    _ = match a {
        11 => 0, 12 if a == b => { 1 }, _ => 0,
        //~^ collapsible_match
    };
}
