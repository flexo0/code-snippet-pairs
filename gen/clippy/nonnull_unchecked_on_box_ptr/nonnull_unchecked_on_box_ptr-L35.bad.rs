fn lint() {
    fn basic() {
        let _ = unsafe {
            //~^ nonnull_unchecked_on_box_ptr
            NonNull::new_unchecked(Box::into_raw(one))
        };
    }
}
