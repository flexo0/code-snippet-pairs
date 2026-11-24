fn example() {
    let y = Some(1);
    while let Some(x) = y {
        consume(x);
    }
}
