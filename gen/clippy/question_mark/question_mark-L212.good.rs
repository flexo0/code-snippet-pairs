fn result_func(x: Result<i32, i32>) -> Result<i32, i32> {
    let _val = func_returning_result()?;
}
