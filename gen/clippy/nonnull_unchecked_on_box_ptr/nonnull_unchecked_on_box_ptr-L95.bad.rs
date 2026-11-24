fn lint() {
    fn macros() {
        let _ = unsafe {
            //~^ nonnull_unchecked_on_box_ptr
            NonNull::new_unchecked(identity!(Box::into_raw(one)))
        };
    }
}
