fn example(v: Vec<usize>) {
    let mut i = 0;
    for item in &v {
        bar(i, *item);
        i += 1;
    }
}
