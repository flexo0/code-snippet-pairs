fn issue13357(ptr: *const [i32], s_ptr: *const &str, a_s_ptr: *const [&str]) {
    unsafe {
        // different types, without erased regions
        let _ = &*(ptr as *const [u32]);
        //~^ transmute_ptr_to_ref
        let _: &[u32] = &*(ptr as *const [u32]);
        //~^ transmute_ptr_to_ref

        // different types, with erased regions
        let _ = &*(a_s_ptr as *const [&[u8]]);
        //~^ transmute_ptr_to_ref
        let _: &[&[u8]] = &*(a_s_ptr as *const [&[u8]]);
        //~^ transmute_ptr_to_ref

        // same type, without erased regions
        let _ = &*(ptr as *const [i32]);
        //~^ transmute_ptr_to_ref
        let _: &[i32] = &*ptr;
        //~^ transmute_ptr_to_ref

        // same type, with erased regions
        let _ = &*(a_s_ptr as *const [&str]);
        //~^ transmute_ptr_to_ref
        let _: &[&str] = &*(a_s_ptr as *const [&str]);
        //~^ transmute_ptr_to_ref
    }
}
