fn iter_nth() {
    {
        // Make sure we lint `.iter()` for relevant types.
        let bad_vec = some_vec.get(3);
        //~^ iter_nth
        let bad_slice = &some_vec[..].get(3);
        //~^ iter_nth
        let bad_boxed_slice = boxed_slice.get(3);
        //~^ iter_nth
        let bad_vec_deque = some_vec_deque.get(3);
        //~^ iter_nth
    }
}
