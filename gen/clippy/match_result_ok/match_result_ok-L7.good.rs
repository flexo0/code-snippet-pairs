fn str_to_int(x: &str) -> i32 {
    if let Ok(y) = x.parse() { y } else { 0 }
}
