fn issue15932() {
    let _: Option<&dyn std::fmt::Debug> = match Some(0) {
        //~^ match_as_ref
        None => None,
        Some(ref mut v) => Some(v),
    };
}
