fn issue14239() {
    let _ = if false {
        None
    } else {
        "1".parse::<u8>().ok()
    };
}
