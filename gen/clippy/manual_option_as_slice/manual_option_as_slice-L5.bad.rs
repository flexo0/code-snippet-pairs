fn check(x: Option<u32>) {
    _ = match x.as_ref() {
        //~^ manual_option_as_slice
        Some(f) => std::slice::from_ref(f),
        None => &[],
    };
}
