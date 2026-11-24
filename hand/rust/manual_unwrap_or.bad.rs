fn example() {
    let foo: Option<i32> = None;
    let v = match foo {
        Some(v) => v,
        None => 1,
    };
}
