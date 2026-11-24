fn issue10808(bar: Option<i32>) {
    match bar {
        Some(v) => unsafe {
            let r = &v as *const i32;
            println!("{}", *r);
        },
        _ => {},
    }
    //~^^^^^^^ single_match

    match bar {
        #[rustfmt::skip]
        Some(v) => {
            unsafe {
                let r = &v as *const i32;
                println!("{}", *r);
            }
        },
        _ => {},
    }
}
