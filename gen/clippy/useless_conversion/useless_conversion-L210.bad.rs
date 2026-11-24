fn explicit_into_iter_fn_arg() {
    b(vec![1, 2].into_iter().into_iter().into_iter());
}
