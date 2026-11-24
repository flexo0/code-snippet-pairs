fn single_match() {
    match z {
        (2..=3, 7..=9) => dummy(),
        _ => {},
    };
}
