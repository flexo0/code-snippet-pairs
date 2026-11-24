fn result_func(x: Result<i32, i32>) -> Result<i32, i32> {
    let _ = x?;
}
