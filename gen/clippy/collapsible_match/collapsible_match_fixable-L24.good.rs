fn issue16558() {
    let _ = match opt {
        Some(s) if s > 2
            && s == 1 => { s }
            //~^ collapsible_match
        ,
        _ => 1,
    };
}
