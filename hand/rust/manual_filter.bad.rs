fn example() {
    let r = match Some(0) {
        Some(x) => if x % 2 == 0 {
            Some(x)
        } else {
            None
        },
        None => None,
    };
}
