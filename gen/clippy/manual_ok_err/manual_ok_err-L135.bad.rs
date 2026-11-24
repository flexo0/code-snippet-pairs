fn issue14239() {
    let _ = if false {
        None
    } else if let Ok(n) = "1".parse::<u8>() {
        Some(n)
    } else {
        None
    };
}
