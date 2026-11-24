fn msrv_1_99() {
    let _ = unsafe {
        //~^ nonnull_unchecked_on_box_ptr
        NonNull::new_unchecked(Box::into_raw(one))
    };
}
