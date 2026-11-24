fn example(v: Vec<usize>) {
    for (i, item) in v.iter().enumerate() {
        bar(i, *item);
    }
}
