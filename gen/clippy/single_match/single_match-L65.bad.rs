fn single_match_know_enum() {
    match y {
        Ok(y) => dummy(),
        Err(..) => (),
    };
}
