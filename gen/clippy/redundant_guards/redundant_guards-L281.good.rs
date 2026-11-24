fn issue11807() {
    match Some(Some(&[] as &[i32])) {
        Some(Some([..])) => {},
        //~^ redundant_guards
        _ => {},
    }
}
