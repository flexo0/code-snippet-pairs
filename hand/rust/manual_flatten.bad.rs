fn example() {
    let x = vec![Some(1), Some(2), Some(3)];
    for n in x {
        if let Some(n) = n {
            println!("{}", n);
        }
    }
}
