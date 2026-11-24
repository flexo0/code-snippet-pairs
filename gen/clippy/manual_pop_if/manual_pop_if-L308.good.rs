fn msrv_high_enough_vec(mut vec: Vec<i32>) {
    vec.pop_if(|x| *x > 2);
}
