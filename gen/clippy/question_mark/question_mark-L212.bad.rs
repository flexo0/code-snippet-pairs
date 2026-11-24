fn result_func(x: Result<i32, i32>) -> Result<i32, i32> {
    let _val = match func_returning_result() {
        //~^ question_mark
        Ok(val) => val,
        Err(err) => return Err(err),
    };
}
