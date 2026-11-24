fn issue13357(ptr: *const [i32], s_ptr: *const &str, a_s_ptr: *const [&str]) {
    unsafe {
        // different types, without erased regions
        let _ = core::mem::transmute::<_, &[u32]>(ptr);
        //~^ transmute_ptr_to_ref
        let _: &[u32] = core::mem::transmute(ptr);
        //~^ transmute_ptr_to_ref

        // different types, with erased regions
        let _ = core::mem::transmute::<_, &[&[u8]]>(a_s_ptr);
        //~^ transmute_ptr_to_ref
        let _: &[&[u8]] = core::mem::transmute(a_s_ptr);
        //~^ transmute_ptr_to_ref

        // same type, without erased regions
        let _ = core::mem::transmute::<_, &[i32]>(ptr);
        //~^ transmute_ptr_to_ref
        let _: &[i32] = core::mem::transmute(ptr);
        //~^ transmute_ptr_to_ref

        // same type, with erased regions
        let _ = core::mem::transmute::<_, &[&str]>(a_s_ptr);
        //~^ transmute_ptr_to_ref
        let _: &[&str] = core::mem::transmute(a_s_ptr);
        //~^ transmute_ptr_to_ref
    }
}
