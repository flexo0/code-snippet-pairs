fn issue11807() {
    match Some(Some("")) {
        Some(Some("")) => {},
        //~^ redundant_guards
        _ => {},
    }
}
