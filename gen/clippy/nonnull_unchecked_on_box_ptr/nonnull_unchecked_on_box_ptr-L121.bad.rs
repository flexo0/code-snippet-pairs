fn lint() {
    fn keep_unsafe_block() {
        let _ = unsafe {
            identity(NonNull::new_unchecked(Box::into_raw(one)))
            //~^ nonnull_unchecked_on_box_ptr
        };
    }
}
