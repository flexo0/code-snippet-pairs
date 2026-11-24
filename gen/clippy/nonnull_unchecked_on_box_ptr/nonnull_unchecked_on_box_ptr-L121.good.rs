fn lint() {
    fn keep_unsafe_block() {
        let _ = unsafe {
            identity(Box::into_non_null(one))
            //~^ nonnull_unchecked_on_box_ptr
        };
    }
}
