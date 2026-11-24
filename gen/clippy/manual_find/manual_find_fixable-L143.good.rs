fn explicit_ret(arr: Vec<i32>) -> Option<i32> {
    arr.into_iter().find(|&x| x >= 5)
}
