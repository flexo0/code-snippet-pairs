fn explicit_ret(arr: Vec<i32>) -> Option<i32> {
    for x in arr {
        //~^ manual_find
        if x >= 5 {
            return Some(x);
        }
    }
    return None;
}
