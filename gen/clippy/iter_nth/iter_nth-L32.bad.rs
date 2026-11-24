fn iter_nth() {
    {
        // Make sure we lint `.iter()` for relevant types.
        let bad_vec = some_vec.iter().nth(3);
        //~^ iter_nth
        let bad_slice = &some_vec[..].iter().nth(3);
        //~^ iter_nth
        let bad_boxed_slice = boxed_slice.iter().nth(3);
        //~^ iter_nth
        let bad_vec_deque = some_vec_deque.iter().nth(3);
        //~^ iter_nth
    }
}
