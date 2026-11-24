fn issue_15679() -> Result<i32, String> {
    match some_result {
        //~^ question_mark
        Ok(val) => val,
        Err(err) => return Err(err.into()),
    };
}
