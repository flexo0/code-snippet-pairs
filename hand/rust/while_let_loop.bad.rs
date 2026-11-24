fn example() {
    let y = Some(1);
    loop {
        let x = match y {
            Some(x) => x,
            None => break,
        };
        consume(x);
    }
}
