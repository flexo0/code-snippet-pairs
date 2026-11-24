fn example() {
    let x = vec![Some(1), Some(2), Some(3)];
    for n in x.into_iter().flatten() {
        println!("{}", n);
    }
}
