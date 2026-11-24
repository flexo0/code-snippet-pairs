fn issue11807() {
    match Some(Some(&[] as &[i32])) {
        Some(Some(x)) if x.starts_with(&[]) => {},
        //~^ redundant_guards
        _ => {},
    }
}
