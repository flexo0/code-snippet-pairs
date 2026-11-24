fn as_closure() {
    let f = |arr: Vec<i32>| -> Option<i32> {
        arr.into_iter().find(|&x| x < 1)
    };
}
