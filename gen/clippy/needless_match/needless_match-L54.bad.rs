fn option_match(x: Option<i32>) {
    let _: Option<i32> = match x {
        //~^ needless_match
        Some(a) => Some(a),
        None => None,
    };
}
