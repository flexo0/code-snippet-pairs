fn should_lint() {
    for i in &mut some_slice {
        //~^ manual_slice_fill
        *i = 0;
    }
}
