fn result_func(x: Result<i32, i32>) -> Result<i32, i32> {
    let _ = if let Ok(x) = x { x } else { return x };
}
