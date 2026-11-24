fn issue10808(bar: Option<i32>) {
    if let Some(v) = bar { unsafe {
        let r = &v as *const i32;
        println!("{}", *r);
    } }
    //~^^^^^^^ single_match

    if let Some(v) = bar {
        unsafe {
            let r = &v as *const i32;
            println!("{}", *r);
        }
    }
}
