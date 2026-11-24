fn msrv_high_enough_vec(mut vec: Vec<i32>) {
    if vec.last().is_some_and(|x| *x > 2) {
        vec.pop().unwrap();
    }
}
