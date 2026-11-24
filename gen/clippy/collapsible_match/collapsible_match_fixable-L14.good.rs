fn issue16558() {
    match opt {
        Some(s)
            if s == 1 => {
                //~^ collapsible_match
                todo!()
            },
        _ => {},
    };
}
