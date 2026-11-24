fn str_to_int(x: &str) -> i32 {
    if let Some(y) = x.parse().ok() { y } else { 0 }
}
