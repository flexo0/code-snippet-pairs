fn lint() {
    fn macros() {
        let _ = unsafe {
            //~^ nonnull_unchecked_on_box_ptr
            identity!(NonNull::new_unchecked(Box::into_raw(one)))
        };
    }
}
