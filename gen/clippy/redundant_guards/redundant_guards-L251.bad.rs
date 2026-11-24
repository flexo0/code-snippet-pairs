fn issue11807() {
    match Some(Some("")) {
        Some(Some(x)) if x.is_empty() => {},
        //~^ redundant_guards
        _ => {},
    }
}
