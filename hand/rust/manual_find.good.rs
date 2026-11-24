fn example(arr: Vec<i32>) -> Option<i32> {
    arr.into_iter().find(|&el| el == 1)
}
