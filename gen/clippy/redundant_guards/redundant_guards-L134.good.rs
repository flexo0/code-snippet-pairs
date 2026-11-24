fn h(v: Option<u32>) {
    match v {
        Some(0) => ..,
        //~^ redundant_guards
        _ => ..,
    };
}
