fn issue16429(b: i32) -> Option<i32> {
    let _ = if b == 1 {
        b
    } else { a? };
}
