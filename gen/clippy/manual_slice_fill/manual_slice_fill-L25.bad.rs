fn should_lint() {
    for i in 0..some_slice.len() {
        //~^ manual_slice_fill
        some_slice[i] = 0;
    }
}
