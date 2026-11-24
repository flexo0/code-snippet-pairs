fn ptr_to_ref<T, U>(p: *const T, m: *mut T, o: *const U, om: *mut U) {
    unsafe {
        let _: &T = &*p;
        //~^ transmute_ptr_to_ref
        let _: &T = &*p;

        let _: &mut T = &mut *m;
        //~^ transmute_ptr_to_ref
        let _: &mut T = &mut *m;

        let _: &T = &*m;
        //~^ transmute_ptr_to_ref
        let _: &T = &*m;

        let _: &mut T = &mut *(p as *mut T);
        //~^ transmute_ptr_to_ref
        let _ = &mut *(p as *mut T);

        let _: &T = &*(o as *const T);
        //~^ transmute_ptr_to_ref
        let _: &T = &*(o as *const T);

        let _: &mut T = &mut *(om as *mut T);
        //~^ transmute_ptr_to_ref
        let _: &mut T = &mut *(om as *mut T);

        let _: &T = &*(om as *const T);
        //~^ transmute_ptr_to_ref
        let _: &T = &*(om as *const T);
    }
}
