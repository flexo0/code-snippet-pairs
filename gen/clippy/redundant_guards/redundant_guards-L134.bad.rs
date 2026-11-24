fn h(v: Option<u32>) {
    match v {
        x if matches!(x, Some(0)) => ..,
        //~^ redundant_guards
        _ => ..,
    };
}
