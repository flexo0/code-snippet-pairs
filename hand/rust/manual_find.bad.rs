fn example(arr: Vec<i32>) -> Option<i32> {
    for el in arr {
        if el == 1 {
            return Some(el);
        }
    }
    None
}
